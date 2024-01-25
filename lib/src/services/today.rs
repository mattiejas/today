use crate::{
    domain::today::{Today, TodayItem},
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
        .map_err(|_| anyhow!("Failed to get today"))?;

        Ok(today)
    }

    pub async fn create_today(&self, user_id: Uuid) -> AppResult<Today> {
        // check if there is no today for the user
        let today = self.get_today().await?;

        if today.is_some() {
            return Err(anyhow!("Today already exists").into());
        }

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
            ORDER BY t.date DESC
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
            "#,
            today_id,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }
}
