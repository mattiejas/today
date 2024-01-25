use std::ops::Add;

use anyhow::anyhow;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    config::JwtConfig,
    domain::user::User,
    error::{AppError, AppResult},
    state::AppState,
};

pub struct JwtService {
    config: JwtConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: Uuid,
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub name: String,
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized(anyhow!("Invalid token").into()))?;

        // Decode the user data
        let state = AppState::from_ref(state);
        let token_data: UserClaims = bearer
            .token()
            .verify_with_key(&get_signing_key(&state.config.jwt)?)
            .map_err(|_| AppError::Unauthorized(anyhow!("Invalid token").into()))?;

        Ok(token_data)
    }
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user: &User) -> AppResult<String> {
        let key = get_signing_key(&self.config)?;

        let expiration = chrono::Utc::now()
            .add(chrono::Duration::seconds(self.config.expiration as i64))
            .timestamp();

        let claims: UserClaims = UserClaims {
            sub: user.id,
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

    pub fn verify_token(&self, token: &str) -> AppResult<Uuid> {
        let key = get_signing_key(&self.config)?;

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

        Ok(claims.sub)
    }
}

fn get_signing_key(jwt: &JwtConfig) -> AppResult<Hmac<Sha256>> {
    let key = Hmac::new_from_slice(jwt.secret.as_ref())
        .map_err(|_| anyhow!("Failed to create JWT signing key"))?;

    Ok(key)
}
