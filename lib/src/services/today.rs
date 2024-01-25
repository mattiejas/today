use crate::{domain::today::Today, error::AppResult};
use anyhow::anyhow;
use sqlx::PgPool;

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

    pub async fn create_today(&self) -> AppResult<Today> {
        let title = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let today = sqlx::query_as!(
            Today,
            r#"
            INSERT INTO today (id, date, title, created_at, updated_at)
            VALUES (gen_random_uuid(), CURRENT_DATE, $1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) 
            RETURNING *
            "#,
            title
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(today)
    }
}
