use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Default, SimpleObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub discord_webhook_url: Option<String>,
}
