use async_graphql::{Object, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, types::Json};

#[derive(Serialize, Deserialize, FromRow, Clone, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Today {
    pub id: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct TodayItem {
    pub id: String,
    pub today_id: String,

    #[graphql(skip)]
    pub content: Json<TodayBlockContent>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TodayBlockContent {
    Text(String),
    Todo(bool),
}

#[Object]
impl TodayBlockContent {
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
