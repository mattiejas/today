use std::sync::Arc;

use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::PgPool;

use crate::cqrs::event::ClanEvent;
use crate::db::Db;

use crate::views;

#[derive(Clone)]
pub struct AppState {
    pub prisma: Db,
    pub db_pool: PgPool,
    pub cqrs: Arc<PostgresCqrs<ClanEvent>>,
    pub clan_event_query: Arc<PostgresViewRepository<views::event::ClanEventView, ClanEvent>>,
}
