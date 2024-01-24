use crate::{
    domain,
    error::{AppError, AppResult},
    prisma,
};
use anyhow::anyhow;
use sqlx::{postgres::PgRow, query, Executor, Row};

pub struct ClanService {
    pub db_pool: sqlx::PgPool,
}

impl ClanService {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

impl ClanService {
    pub async fn create(&self, name: &str, display_name: &str) -> AppResult<prisma::clan::Data> {
        let db = crate::db::new().await?;

        if self.exists_by_name(name).await? {
            return Err(AppError::BadRequest(
                anyhow!("Clan with name `{}` already exists", name).into(),
            ));
        }

        let clan = db
            .clan()
            .create(display_name.to_string(), name.to_string(), vec![])
            .exec()
            .await?;

        Ok(clan.into())
    }

    pub async fn all(&self) -> AppResult<Vec<domain::clan::Clan>> {
        let clans = sqlx::query(
            r#"
            SELECT id, name, display_name, discord_webhook_url
            FROM clans
            "#,
        )
        .map(|row: PgRow| domain::clan::Clan {
            id: row.get(0),
            name: row.get(1),
            display_name: row.get(2),
            discord_webhook_url: row.get(3),
        })
        .fetch_all(&self.db_pool)
        .await?;

        Ok(clans)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<Option<domain::clan::Clan>> {
        let clan = sqlx::query(
            r#"
            SELECT id, name, display_name, discord_webhook_url
            FROM clans
            WHERE id = $1
            "#,
        )
        .bind(id)
        .map(|row: PgRow| domain::clan::Clan {
            id: row.get(0),
            name: row.get(1),
            display_name: row.get(2),
            discord_webhook_url: row.get(3),
        })
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(clan)
    }

    pub async fn exists(&self, id: i32) -> AppResult<bool> {
        let result = self
            .db_pool
            .execute(query!(r#"SELECT id FROM clans WHERE id = $1"#, id))
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn exists_by_name(&self, name: &str) -> AppResult<bool> {
        let result = self
            .db_pool
            .execute(query!(r#"SELECT id FROM clans WHERE name = $1"#, name))
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
