use async_graphql::SimpleObject;
use cqrs_es::{persist::GenericQuery, View};
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use crate::cqrs::event::{ClanEvent, ClanEventDomainEvent};

pub type ClanEventViewQuery =
    GenericQuery<PostgresViewRepository<ClanEventView, ClanEvent>, ClanEventView, ClanEvent>;

#[derive(Debug, Clone, Default, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct ClanEventView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub starts_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    pub members: Vec<String>,
    pub clan_id: i32,
}

impl View<ClanEvent> for ClanEventView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<ClanEvent>) {
        match &event.payload {
            ClanEventDomainEvent::Created { name, clan_id } => {
                self.id = event.aggregate_id.clone();
                self.name = name.clone();
                self.clan_id = *clan_id;
            }
        }
    }
}
