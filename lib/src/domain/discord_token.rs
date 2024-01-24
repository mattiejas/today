use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscordToken {
    pub id: i32,
    pub discord_id: String,
    pub discord_username: String,
    pub expire_at: Option<NaiveDateTime>,
    pub access_token: String,
    pub last_refresh_at: NaiveDate,
    pub refresh_token: String,
    pub user_id: i32,
}
