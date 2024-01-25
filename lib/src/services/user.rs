use crate::{
    domain::{self},
    error::AppResult,
};

use anyhow::anyhow;

pub struct UserService {
    pub db_pool: sqlx::PgPool,
}

impl UserService {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> AppResult<domain::user::User> {
        // check if username or email already exists
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE username = $1 OR email = $2
            ) AS "exists!: bool"
            "#,
            username,
            email
        )
        .fetch_one(&self.db_pool)
        .await?
        .exists;

        if exists {
            return Err(anyhow!("Username or email already exists").into());
        }

        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|_| anyhow!("Failed to hash password"))?;

        let user = sqlx::query_as!(
            domain::user::User,
            r#"
            INSERT INTO users (id, username, email, password)
            VALUES (gen_random_uuid(), $1, $2, $3)
            RETURNING id, username, email
            "#,
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(user)
    }

    pub async fn login(&self, email: String, password: String) -> AppResult<domain::user::User> {
        let user = sqlx::query_as!(
            domain::user::UserWithPassword,
            r#"
            SELECT id, username, email, password as "password_hash: String"
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.db_pool)
        .await?;

        let valid = bcrypt::verify(password, &user.password_hash)
            .map_err(|_| anyhow!("Failed to verify password"))?;

        if !valid {
            return Err(anyhow!("Invalid password").into());
        }

        Ok(domain::user::User {
            id: user.id,
            username: user.username,
            email: user.email,
        })
    }
}
