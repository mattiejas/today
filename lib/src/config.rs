use anyhow::anyhow;
use dotenvy::dotenv;

use crate::error::AppResult;

#[derive(Clone)]
pub struct Config {
    pub db_url: String,
    pub jwt: JwtConfig,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub expiration: i64,
}

impl Config {
    pub fn new() -> AppResult<Self> {
        dotenv().ok();

        let db_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow!("The environment variable DATABASE_URL must be set"))?;

        let jwt = Self::parse_jwt_config()?;

        Ok(Self { db_url, jwt })
    }

    fn parse_jwt_config() -> AppResult<JwtConfig> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| anyhow!("The environment variable JWT_SECRET must be set"))?;

        let issuer = std::env::var("JWT_ISSUER")
            .map_err(|_| anyhow!("The environment variable JWT_ISSUER must be set"))?;

        let audience = std::env::var("JWT_AUDIENCE")
            .map_err(|_| anyhow!("The environment variable JWT_AUDIENCE must be set"))?;

        let expiration = std::env::var("JWT_EXPIRATION")
            .map_err(|_| anyhow!("The environment variable JWT_EXPIRATION must be set"))?;

        let expiration = expiration
            .parse::<i64>()
            .map_err(|_| anyhow!("The environment variable JWT_EXPIRATION must be an integer"))?;

        Ok(JwtConfig {
            secret,
            issuer,
            audience,
            expiration,
        })
    }
}
