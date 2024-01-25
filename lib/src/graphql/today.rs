use crate::{
    domain::{self, today::Today},
    error::AppResult,
    services::AppServices,
};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TodayQuery;

#[Object]
impl TodayQuery {
    async fn clans(&self, ctx: &Context<'_>) -> AppResult<Vec<Today>> {
        let services = ctx.data::<AppServices>()?;

        todo!()
    }

    async fn clan(&self, ctx: &Context<'_>, id: i32) -> AppResult<Option<Today>> {
        let services = ctx.data::<AppServices>()?;

        todo!()
    }
}
