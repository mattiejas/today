



pub struct EventService {
    pub db_pool: sqlx::PgPool,
}

impl EventService {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

impl EventService {}
