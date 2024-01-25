use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, types::JsonValue};

use crate::utils::uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct Today {
    pub id: Uuid,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, SimpleObject)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct TodayItem {
    pub id: Uuid,
    pub today_id: Uuid,

    #[graphql(skip)]
    pub content: TodayBlockContent,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "payload")]
pub enum TodayBlockContent {
    Text(String),
    Todo(bool),
}

impl From<JsonValue> for TodayBlockContent {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

#[Object]
impl TodayBlockContent {
    #[graphql(name = "type")]
    async fn type_name(&self) -> String {
        match self {
            TodayBlockContent::Text(_) => "TEXT".to_string(),
            TodayBlockContent::Todo(_) => "TODO".to_string(),
        }
    }

    async fn payload(&self) -> String {
        match self {
            TodayBlockContent::Text(text) => text.clone(),
            TodayBlockContent::Todo(done) => done.to_string(),
        }
    }
}

#[ComplexObject]
impl Today {
    pub async fn items(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<Vec<TodayItem>> {
        let services = ctx.data::<crate::services::AppServices>()?;
        let claims = ctx.data::<crate::services::jwt::UserClaims>()?;

        let items = services.today.get_items(claims.sub, self.id.into()).await?;

        Ok(items)
    }
}

#[ComplexObject]
impl TodayItem {
    async fn content(&self) -> String {
        match &self.content {
            TodayBlockContent::Text(text) => text.clone(),
            TodayBlockContent::Todo(done) => done.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct TodayItemContentInput {
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub type_name: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct UpsertTodayItemObject {
    pub today_id: Uuid,
    pub today_item_id: Option<Uuid>,
    pub content: TodayItemContentInput,
}
