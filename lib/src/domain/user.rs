use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, types::Uuid};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithPassword {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
