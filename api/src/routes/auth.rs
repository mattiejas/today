use anyhow::anyhow;
use axum::{
    extract::State,
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
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
        .route("/auth/me", get(get_logged_in_user))
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
) -> AppResult<impl IntoResponse> {
    let user = services
        .user
        .create_user(req.username, req.email, req.password)
        .await?;

    // generate jwt token
    let jwt_token = services.jwt.generate_token(&user)?;

    // set cookie
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&format!("jwt={}", jwt_token))
            .map_err(|_| -> AppError { anyhow!("Failed to generate cookie").into() })?,
    );

    Ok((headers, jwt_token))
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(
    services: Extension<AppServices>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let user = services
        .user
        .login(req.username, req.password)
        .await
        .map_err(|_| AppError::Unauthorized(anyhow!("Invalid email or password").into()))?;

    // generate jwt token
    let jwt_token = services.jwt.generate_token(&user)?;

    // set cookie
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&format!("jwt={}", jwt_token))
            .map_err(|_| -> AppError { anyhow!("Failed to generate cookie").into() })?,
    );

    Ok((headers, jwt_token))
}

async fn get_logged_in_user(
    claims: lib::services::jwt::UserClaims,
    State(_): State<AppState>,
    services: Extension<AppServices>,
) -> AppResult<Json<lib::domain::user::User>> {
    let user = services.user.get_user_by_id(claims.sub).await?;

    Ok(Json(user))
}
