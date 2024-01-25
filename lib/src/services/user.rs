use crate::{
    domain::{self},
    error::AppResult,
};

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
        hashed_password: String,
    ) -> AppResult<domain::user::User> {
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
}
