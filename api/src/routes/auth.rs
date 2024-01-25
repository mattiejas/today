use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use chrono::NaiveDateTime;
use lib::{
    error::{AppError, AppResult},
    services::AppServices,
    state::AppState,
};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
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

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login(
    services: Extension<AppServices>,
    Json(req): Json<LoginRequest>,
) -> AppResult<String> {
    log::info!("Login request: {:?}, {:?}", req.email, req.password);
    let user = services.user.login(req.email, req.password).await?;
    // .map_err(|_| AppError::Unauthorized(anyhow!("Invalid email or password").into()))?;

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
