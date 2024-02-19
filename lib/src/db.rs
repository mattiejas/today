use postgres_es::default_postgress_pool;

use crate::{config::Config, error::AppResult};

pub type DbPool = sqlx::Pool<sqlx::Postgres>;

pub async fn new_pool() -> AppResult<DbPool> {
    let config = Config::new()?;

    let pool = default_postgress_pool(config.db_url.as_str()).await;

    sqlx::migrate!("../migrations").run(&pool).await
        .expect("Failed to run migrations");

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dotenv_is_read() {
        let config = Config::new();
        assert!(config.is_ok());
    }
}
