use std::sync::Arc;

use cqrs_es::{persist::PersistedEventStore, Query};
use postgres_es::{PostgresCqrs, PostgresEventRepository, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::{
    services::AppServices,
    views::event::{ClanEventView, ClanEventViewQuery},
};

use self::event::ClanEvent;

pub mod event;

pub fn cqrs_framework(
    pool: Pool<Postgres>,
    services: AppServices,
) -> (
    Arc<PostgresCqrs<ClanEvent>>,
    Arc<PostgresViewRepository<ClanEventView, ClanEvent>>,
) {
    let clan_event_view_repo = Arc::new(PostgresViewRepository::new(
        "clan_events_view",
        pool.clone(),
    ));

    let mut clan_event_query = ClanEventViewQuery::new(clan_event_view_repo.clone());

    clan_event_query.use_error_handler(Box::new(|error| {
        log::error!("Error in ClanEventQuery: {}", error);
    }));

    let queries: Vec<Box<dyn Query<ClanEvent>>> = vec![Box::new(clan_event_query)];

    let repository =
        PostgresEventRepository::new(pool.clone()).with_tables("source_events", "snapshots");

    let store = PersistedEventStore::new_event_store(repository);

    let cqrs = Arc::new(PostgresCqrs::new(store, queries, services));

    (cqrs, clan_event_view_repo)
}
