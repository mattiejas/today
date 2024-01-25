use async_graphql::{Object, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Today {
    pub id: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub blocks: Vec<TodayBlock>,
}

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct TodayBlock {
    pub id: String,
    pub content: TodayBlockContent,
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
