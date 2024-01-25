use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
pub struct TodayItem {
    pub id: Uuid,
    pub today_id: Uuid,

    pub content: TodayBlockContent,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "payload")]
pub enum TodayBlockContent {
    Text(String),
    Todo {
        text: String,
        #[serde(rename = "isCompleted")]
        is_completed: bool,
    },
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
            TodayBlockContent::Text(_) => "text".to_string(),
            TodayBlockContent::Todo { .. } => "todo".to_string(),
        }
    }

    async fn payload(&self) -> JsonValue {
        match self {
            TodayBlockContent::Text(text) => json!(text),
            TodayBlockContent::Todo { text, is_completed } => {
                json!({ "text": text, "isCompleted": is_completed })
            }
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
