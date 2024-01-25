use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use chrono::NaiveDateTime;
use lib::{error::AppResult, services::AppServices, state::AppState};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/register", post(register))
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

async fn register(
    services: Extension<AppServices>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<String> {
    let hashed_password = bcrypt::hash(req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| anyhow!("Failed to hash password"))?;

    let user = services
        .user
        .create_user(req.username, req.email, hashed_password)
        .await?;

    // generate jwt token
    let jwt_token = services.jwt.generate_token(&user)?;

    Ok(jwt_token)
}

struct RefreshTokenRequest {
    access_token: String,
    refresh_token: String,
}

struct RefreshTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_at: NaiveDateTime,
}

async fn refresh_token(
    services: Extension<AppServices>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<RefreshTokenResponse> {
    // is the discord token still valid?
    // otherwise deny the request to refresh the token

    Ok(RefreshTokenResponse {
        access_token: "".to_string(),
        refresh_token: "".to_string(),
        expires_at: chrono::Utc::now().naive_utc(),
    })
}
