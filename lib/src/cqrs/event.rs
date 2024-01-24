use anyhow::anyhow;
use async_trait::async_trait;
use cqrs_es::{Aggregate, DomainEvent};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, services::AppServices};

#[derive(Debug, Deserialize)]
pub enum ClanEventCommand {
    CreateEvent { name: String, clan_id: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClanEventDomainEvent {
    Created { name: String, clan_id: i32 },
}

impl DomainEvent for ClanEventDomainEvent {
    fn event_type(&self) -> String {
        match self {
            ClanEventDomainEvent::Created { .. } => "Created".to_string(),
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ClanEvent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub visibility: String,
    pub starts_at: chrono::DateTime<chrono::Utc>,
    pub ends_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub clan_id: i32,
    pub board_id: String,
}

#[async_trait]
impl Aggregate for ClanEvent {
    type Command = ClanEventCommand;
    type Event = ClanEventDomainEvent;
    type Error = AppError;
    type Services = AppServices;

    fn aggregate_type() -> String {
        "ClanEvent".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            ClanEventCommand::CreateEvent { name, clan_id } => {
                match services.clan.exists(clan_id).await {
                    Ok(exists) => {
                        if !exists {
                            return Err(AppError::BadRequest(
                                anyhow!("Clan with id {} does not exist", clan_id).into(),
                            ));
                        }
                    }
                    Err(error) => {
                        return Err(anyhow!("Failed to check if clan exists: {}", error).into());
                    }
                }

                let event = ClanEventDomainEvent::Created { name, clan_id };
                Ok(vec![event])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ClanEventDomainEvent::Created { name, clan_id } => {
                self.clan_id = clan_id;
                self.name = name;
                self.created_at = chrono::Utc::now();
                self.updated_at = chrono::Utc::now();
            }
        }
    }
}
