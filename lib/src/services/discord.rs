use serde::{Deserialize, Serialize};

use crate::error::AppResult;

const DISCORD_AUTH_URL: &str = "https://discord.com/oauth2/authorize";
const DISCORD_TOKEN_URL: &str = "https://discord.com/api/oauth2/token";

#[derive(Clone)]
pub struct DiscordConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub struct DiscordService {
    config: DiscordConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordUserResponse {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub premium_type: Option<u32>,
    pub public_flags: Option<u32>,
    pub avatar_decoration: Option<String>,
}

impl DiscordService {
    pub fn new(config: DiscordConfig) -> Self {
        Self { config }
    }

    pub fn get_auth_url(&self) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=identify",
            DISCORD_AUTH_URL, self.config.client_id, self.config.redirect_uri
        )
    }

    pub async fn redeem_code(&self, code: String) -> AppResult<DiscordTokenResponse> {
        let client = reqwest::Client::new();

        let res = client
            .post(DISCORD_TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&[
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.as_str()),
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", self.config.redirect_uri.as_str()),
                ("scope", "identify"),
            ])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        let token = res
            .json::<DiscordTokenResponse>()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

        Ok(token)
    }

    pub async fn refresh_token(&self, refresh_token: String) -> AppResult<DiscordTokenResponse> {
        let client = reqwest::Client::new();

        let res = client
            .post(DISCORD_TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&[
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.as_str()),
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token.as_str()),
                ("redirect_uri", self.config.redirect_uri.as_str()),
                ("scope", "identify"),
            ])
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        let token = res
            .json::<DiscordTokenResponse>()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

        Ok(token)
    }

    pub async fn get_user(&self, token: &str) -> AppResult<DiscordUserResponse> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://discord.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        let user = res
            .json::<DiscordUserResponse>()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

        Ok(user)
    }
}
