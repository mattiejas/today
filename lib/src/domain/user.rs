use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub name: String,
    pub avatar_hash: Option<String>,
}
