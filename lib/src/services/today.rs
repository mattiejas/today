use crate::{
    domain::today::{Today, TodayBlockContent, TodayItem},
    error::AppResult,
};
use anyhow::anyhow;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TodayService {
    pub db_pool: PgPool,
}

impl TodayService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_today(&self) -> AppResult<Option<Today>> {
        let today = sqlx::query_as!(
            Today,
            r#"
            SELECT *
            FROM today t
            WHERE t.date = CURRENT_DATE 
            "#
        )
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| anyhow!("Failed to get today by date"))?;

        Ok(today)
    }

    pub async fn get_today_by_id(&self, today_id: Uuid) -> AppResult<Option<Today>> {
        let today = sqlx::query_as!(
            Today,
            r#"
            SELECT *
            FROM today t
            WHERE t.id = $1
            "#,
            today_id
        )
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|_| anyhow!("Failed to get today with ID '{}'", today_id))?;

        Ok(today)
    }

    pub async fn create_today(&self, user_id: Uuid) -> AppResult<Today> {
        let title = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let today = sqlx::query_as!(
            Today,
            r#"
            INSERT INTO today (id, date, title, created_at, updated_at, user_id)
            VALUES (gen_random_uuid(), CURRENT_DATE, $1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $2)
            RETURNING *
            "#,
            title,
            user_id
        )
            .fetch_one(&self.db_pool)
            .await?;

        let content = TodayBlockContent::Text("".to_string());

        self.insert_item(UpsertTodayItem {
            user_id,
            today_id: today.id.into(),
            insert_at: Some(0),
            today_item_id: None,
            content: serde_json::json!(content),
        })
            .await?;

        Ok(today)
    }

    pub async fn get_history(
        &self,
        user_id: Uuid,
        page: usize,
        limit: usize,
    ) -> AppResult<Vec<Today>> {
        let history = sqlx::query_as!(
            Today,
            r#"
            SELECT *
            FROM today t
            WHERE t.user_id = $1
            ORDER BY t.created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            user_id,
            limit as i64,
            (page * limit) as i64
        )
            .fetch_all(&self.db_pool)
            .await?;

        Ok(history)
    }

    pub async fn get_items(&self, user_id: Uuid, today_id: Uuid) -> AppResult<Vec<TodayItem>> {
        let items = sqlx::query_as!(
            TodayItem,
            r#"
            SELECT ti.* 
            FROM today_items ti
            LEFT JOIN today t ON t.id = ti.today_id
            WHERE t.user_id = $1 AND t.id = $2
            ORDER BY ti.sort_order ASC
            "#,
            user_id,
            today_id
        )
            .fetch_all(&self.db_pool)
            .await?;

        Ok(items)
    }

    pub async fn upsert_item(&self, dto: UpsertTodayItem) -> AppResult<TodayItem> {
        match dto.today_item_id {
            Some(id) => self.update_item(id, dto).await,
            None => self.insert_item(dto).await,
        }
    }

    pub async fn delete_item(&self, user_id: Uuid, today_item_id: Uuid) -> AppResult<()> {
        let item = sqlx::query!(
            r#"
            DELETE FROM today_items ti
            USING today t
            WHERE ti.id = $1 AND t.user_id = $2 AND ti.today_id = t.id
            "#,
            today_item_id,
            user_id
        )
            .execute(&self.db_pool)
            .await?;

        if item.rows_affected() == 0 {
            return Err(crate::error::AppError::NotFound(
                anyhow!("Today item not found with id {}", today_item_id).into(),
            ));
        }

        Ok(())
    }

    async fn update_item(&self, id: Uuid, dto: UpsertTodayItem) -> AppResult<TodayItem> {
        // begin transaction
        let mut tx = self.db_pool.begin().await?;

        let item = sqlx::query_as!(
            TodayItem,
            r#"
            UPDATE today_items
            SET content = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING *
            "#,
            dto.content,
            id
        )
            .fetch_one(&mut *tx)
            .await?;

        if let Some(insert_at) = dto.insert_at {
            sqlx::query!(
                r#"
                UPDATE today_items
                SET sort_order = sort_order + 1
                WHERE today_id = $1 AND sort_order >= $2
                "#,
                dto.today_id,
                insert_at
            )
                .execute(&mut *tx)
                .await?;

            // update the sort order of the item
            sqlx::query!(
                r#"
                UPDATE today_items
                SET sort_order = $1
                WHERE id = $2
                "#,
                insert_at,
                id
            )
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(item)
    }

    async fn insert_item(&self, dto: UpsertTodayItem) -> AppResult<TodayItem> {
        let mut tx = self.db_pool.begin().await?;

        let sort_order = match dto.insert_at {
            Some(insert_at) => insert_at,
            None => {
                let max_sort_order = sqlx::query!(
                    r#"
                    SELECT MAX(sort_order) as max_sort_order
                    FROM today_items
                    WHERE today_id = $1
                    "#,
                    dto.today_id
                )
                    .fetch_one(&mut *tx)
                    .await?;

                max_sort_order.max_sort_order.unwrap_or(0) + 1
            }
        };

        // update the sort order of the item
        sqlx::query!(
            r#"
            UPDATE today_items
            SET sort_order = sort_order + 1
            WHERE today_id = $1 AND sort_order >= $2
            "#,
            dto.today_id,
            sort_order
        )
            .execute(&mut *tx)
            .await?;

        let item = sqlx::query_as!(
            TodayItem,
            r#"
            INSERT INTO today_items (id, today_id, sort_order, content, created_at, updated_at)
            VALUES (gen_random_uuid(), $1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING *
            "#,
            dto.today_id,
            sort_order,
            dto.content
        )
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(item)
    }
}

pub struct UpsertTodayItem {
    pub user_id: Uuid,
    pub today_id: Uuid,
    pub insert_at: Option<i32>,
    pub today_item_id: Option<Uuid>,
    pub content: serde_json::Value,
}
