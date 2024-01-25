use crate::{domain::today::Today, error::AppResult, services::AppServices};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TodayQuery;

#[Object]
impl TodayQuery {
    async fn today(&self, ctx: &Context<'_>) -> AppResult<Option<Today>> {
        let services = ctx.data::<AppServices>()?;

        let today = services.today.get_today().await?;

        Ok(today)
    }
}

#[derive(Default)]
pub struct TodayMutation;

#[Object]
impl TodayMutation {
    async fn create_today(&self, ctx: &Context<'_>) -> AppResult<Today> {
        let services = ctx.data::<AppServices>()?;

        let today = services.today.create_today().await?;

        Ok(today)
    }
}
