use std::sync::Arc;

use postgres_es::PostgresCqrs;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
