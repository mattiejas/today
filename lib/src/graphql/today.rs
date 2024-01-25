use crate::{
    domain::today::{Today, TodayBlockContent, TodayItem},
    error::AppResult,
    services::{today::UpsertTodayItem, AppServices},
    utils::uuid::Uuid,
};
use async_graphql::{Context, InputObject, Json, Object};
use jwt::claims;
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

    async fn upsert_item(
        &self,
        ctx: &Context<'_>,
        today_id: Uuid,
        content: Json<TodayBlockContent>,
        today_item_id: Option<Uuid>,
    ) -> AppResult<TodayItem> {
        let services = ctx.data::<AppServices>()?;
        let claims = ctx.data::<crate::services::jwt::UserClaims>()?;

        log::info!("Updating item for user {}", claims.sub);

        let today = services
            .today
            .upsert_item(UpsertTodayItem {
                user_id: claims.sub,
                today_id: today_id.into(),
                today_item_id: today_item_id.map(|id| id.into()),
                content: serde_json::json!(content.0),
            })
            .await?;

        Ok(today)
    }

    async fn delete_item(&self, ctx: &Context<'_>, today_item_id: Uuid) -> AppResult<bool> {
        let services = ctx.data::<AppServices>()?;
        let claims = ctx.data::<crate::services::jwt::UserClaims>()?;

        log::info!("Deleting item for user {}", claims.sub);

        services
            .today
            .delete_item(claims.sub, today_item_id.into())
            .await?;

        Ok(true)
    }
}
