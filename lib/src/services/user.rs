use crate::{
    domain::{self, discord_token::DiscordToken, user::User},
    error::AppResult,
};

use super::discord::DiscordUserResponse;

pub struct UserService {
    pub db_pool: sqlx::PgPool,
}

impl UserService {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_or_create_user_from_discord(
        &self,
        discord_user: DiscordUserResponse,
    ) -> AppResult<domain::user::User> {
        let user = sqlx::query_as!(
            domain::user::User,
            r#"
            INSERT INTO users (discord_id, name, avatar_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (discord_id)
            DO UPDATE SET name = $2, avatar_hash = $3
            RETURNING *
            "#,
            discord_user.id,
            discord_user.username,
            discord_user.avatar,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(user)
    }

    pub async fn exists_by_discord_id(&self, discord_id: &str) -> AppResult<bool> {
        let exists = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE discord_id = $1
            )
            "#,
        )
        .bind(discord_id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(exists)
    }

    pub async fn upsert_discord_token(
        &self,
        discord_token: &DiscordToken,
        user: &User,
    ) -> AppResult<DiscordToken> {
        let discord_token = sqlx::query_as(
            r#"
            INSERT INTO discord_tokens (discord_id, discord_username, expire_at, access_token, last_refresh_at, refresh_token, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (discord_id)
            DO UPDATE SET discord_username = $2, expire_at = $3, access_token = $4, last_refresh_at = $5, refresh_token = $6, user_id = $7
            RETURNING *
            "#,
        )
        .bind(discord_token.discord_id.clone())
        .bind(discord_token.discord_username.clone())
        .bind(discord_token.expire_at)
        .bind(discord_token.access_token.clone())
        .bind(discord_token.last_refresh_at)
        .bind(discord_token.refresh_token.clone())
        .bind(user.id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(discord_token)
    }

    pub async fn get_discord_token(&self, discord_id: &str) -> AppResult<Option<DiscordToken>> {
        let discord_token: Option<DiscordToken> = sqlx::query_as(
            r#"
            SELECT *
            FROM discord_tokens
            WHERE discord_id = $1
            "#,
        )
        .bind(discord_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(discord_token)
    }
}
