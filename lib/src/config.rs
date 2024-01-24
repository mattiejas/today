use anyhow::anyhow;
use dotenvy::dotenv;

use crate::{
    error::AppResult,
    services::{discord::DiscordConfig, jwt::JwtConfig},
};

#[derive(Clone)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> AppResult<Self> {
        dotenv().ok();

        let db_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow!("The environment variable DATABASE_URL must be set"))?;

        Ok(Self { db_url })
    }
}
