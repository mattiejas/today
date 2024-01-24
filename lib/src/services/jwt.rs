use std::{ops::Add};

use anyhow::anyhow;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{domain::user::User, error::AppResult};

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub audience: String,
    pub issuer: String,
    pub expiration: u64,
}

pub struct JwtService {
    config: JwtConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: i32,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub name: String,
    pub avatar_hash: Option<String>,
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
            sub: user.id,
            aud: self.config.audience.clone(),
            iss: self.config.issuer.clone(),
            exp: expiration,
            name: user.name.clone(),
            avatar_hash: user.avatar_hash.clone(),
        };

        let token_str = claims
            .sign_with_key(&key)
            .map_err(|_| anyhow!("Failed to sign JWT"))?;

        Ok(token_str)
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Option<i32>> {
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

        Ok(Some(claims.sub))
    }

    fn get_signing_key(&self) -> AppResult<Hmac<Sha256>> {
        let key = Hmac::new_from_slice(self.config.secret.as_ref())
            .map_err(|_| anyhow!("Failed to create JWT signing key"))?;

        Ok(key)
    }
}
