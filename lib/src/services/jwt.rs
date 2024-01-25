use std::ops::Add;

use anyhow::anyhow;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::types::Uuid;

use crate::{config::JwtConfig, domain::user::User, error::AppResult};

pub struct JwtService {
    config: JwtConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub name: String,
    pub email: String,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user: &User) -> AppResult<String> {
        let key = self.get_signing_key()?;

        let expiration = chrono::Utc::now()
            .add(chrono::Duration::seconds(self.config.expiration as i64))
            .timestamp();

        let claims: UserClaims = UserClaims {
            sub: user.id.to_string(),
            aud: self.config.audience.clone(),
            iss: self.config.issuer.clone(),
            exp: expiration,
            name: user.username.clone(),
            email: user.email.clone(),
        };

        let token_str = claims
            .sign_with_key(&key)
            .map_err(|_| anyhow!("Failed to sign JWT"))?;

        Ok(token_str)
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Option<Uuid>> {
        let key = self.get_signing_key()?;

        let claims: UserClaims = token
            .verify_with_key(&key)
            .map_err(|_| anyhow!("Failed to decode JWT"))?;

        if claims.aud != self.config.audience.to_string()
            || claims.iss != self.config.issuer.to_string()
        {
            return Err(anyhow!("Invalid audience or issuer").into());
        }

        let now = chrono::Utc::now().timestamp();

        if now > claims.exp {
            return Err(anyhow!("Token expired").into());
        }

        if let Ok(uuid) = Uuid::parse_str(&claims.sub) {
            return Ok(Some(uuid));
        }

        Err(anyhow!("Invalid user id").into())
    }

    fn get_signing_key(&self) -> AppResult<Hmac<Sha256>> {
        let key = Hmac::new_from_slice(self.config.secret.as_ref())
            .map_err(|_| anyhow!("Failed to create JWT signing key"))?;

        Ok(key)
    }
}
