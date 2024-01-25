use crate::{domain::today::Today, error::AppResult, services::AppServices};
use async_graphql::{Context, InputObject, Object};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, InputObject, Default)]
pub struct Pagination {
    pub page: usize,
    pub limit: usize,
}

#[derive(Default)]
pub struct TodayQuery;

#[Object]
impl TodayQuery {
    async fn today(&self, ctx: &Context<'_>) -> AppResult<Option<Today>> {
        let services = ctx.data::<AppServices>()?;

        let today = services.today.get_today().await?;

        Ok(today)
    }

    async fn history(&self, ctx: &Context<'_>, pagination: Pagination) -> AppResult<Vec<Today>> {
        let services = ctx.data::<AppServices>()?;
        let claims = ctx.data::<crate::services::jwt::UserClaims>()?;

        log::info!("Getting history for user {}", claims.sub);

        let history = services
            .today
            .get_history(claims.sub, pagination.page, pagination.limit)
            .await?;

        Ok(history)
    }
}

#[derive(Default)]
pub struct TodayMutation;

#[Object]
impl TodayMutation {
    async fn create_today(&self, ctx: &Context<'_>) -> AppResult<Today> {
        let services = ctx.data::<AppServices>()?;
        let claims = ctx.data::<crate::services::jwt::UserClaims>()?;

        log::info!("Creating today for user {}", claims.sub);

        let today = services.today.create_today(claims.sub).await?;

        Ok(today)
    }
}
