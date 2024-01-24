use crate::{domain, error::AppResult, services::AppServices};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct ClanQuery;

#[Object]
impl ClanQuery {
    async fn clans(&self, ctx: &Context<'_>) -> AppResult<Vec<domain::clan::Clan>> {
        let services = ctx.data::<AppServices>()?;

        Ok(services.clan.all().await?)
    }

    async fn clan(&self, ctx: &Context<'_>, id: i32) -> AppResult<Option<domain::clan::Clan>> {
        let services = ctx.data::<AppServices>()?;

        Ok(services.clan.find_by_id(id).await?)
    }
}
