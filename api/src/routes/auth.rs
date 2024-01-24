use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::NaiveDateTime;
use lib::{
    domain::discord_token::DiscordToken, error::AppResult, services::AppServices, state::AppState,
};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/login", post(redeem_code))
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn redeem_code(
    services: Extension<AppServices>,
    Json(req): Json<LoginRequest>,
) -> AppResult<String> {
    let response = services.discord.redeem_code(req.code).await?;

    let discord_user = services.discord.get_user(&response.access_token).await?;

    // user is logged in
    let user = services
        .user
        .get_or_create_user_from_discord(discord_user.clone())
        .await?;

    let now = chrono::Utc::now().naive_utc();
    let expires_in = chrono::Duration::seconds(response.expires_in as i64);
    let expire_at = now + expires_in;

    // create discord token for user in our database
    let discord_token = DiscordToken {
        access_token: response.access_token.clone(),
        refresh_token: response.refresh_token.clone(),
        discord_id: discord_user.id,
        discord_username: discord_user.username,
        expire_at: Some(expire_at),
        user_id: user.id,
        last_refresh_at: now.date(),
        id: 0,
    };

    services
        .user
        .upsert_discord_token(&discord_token, &user)
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
        expires_at: chrono::Utc::now(),
    })
}
