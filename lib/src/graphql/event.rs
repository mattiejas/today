use crate::{
    domain,
    error::{AppError, AppResult},
    state::AppState,
    views::event::ClanEventView,
};
use async_graphql::{ComplexObject, Context, Object};
use cqrs_es::persist::ViewRepository;

#[derive(Default)]
pub struct EventQuery;

#[Object]
impl EventQuery {
    async fn event(&self, ctx: &Context<'_>, id: String) -> AppResult<ClanEventView> {
        let state = ctx.data::<AppState>()?;

        let event =
            state.clan_event_query.load(&id).await.map_err(|e| {
                anyhow::anyhow!("Failed to load event with id: {}, error: {}", &id, e)
            })?;

        if let Some(event) = event {
            Ok(event)
        } else {
            Err(AppError::NotFound(
                anyhow::anyhow!("Failed to load event with id: {}", &id).into(),
            ))
        }
    }
}

#[ComplexObject]
impl ClanEventView {
    pub async fn clan(&self, ctx: &Context<'_>) -> AppResult<Option<domain::clan::Clan>> {
        let services = ctx.data::<crate::services::AppServices>()?;
        let clan = services.clan.find_by_id(self.clan_id).await?;

        Ok(clan)
    }
}
