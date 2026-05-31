use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Discord API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Discord rejected the access token (HTTP {0})")]
    InvalidToken(u16),

    #[error("DISCORD_CLIENT_ID must be set")]
    MissingClientId,

    #[error("Access token is for a different client ID")]
    InvalidClientId,

    #[error("Access token has expired")]
    ExpiredToken,
}

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscordApplication {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2MeResponse {
    pub application: DiscordApplication,
    pub user: DiscordUser,
    pub expires: DateTime<Utc>,
}

pub async fn fetch_discord_user(access_token: &str) -> Result<DiscordUser, OAuthError> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://discord.com/api/oauth2/@me")
        .bearer_auth(access_token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(OAuthError::InvalidToken(response.status().as_u16()));
    }

    let oauth_res = response.json::<OAuth2MeResponse>().await?;

    let expected_client_id =
        std::env::var("DISCORD_CLIENT_ID").map_err(|_| OAuthError::MissingClientId)?;

    if oauth_res.application.id != expected_client_id {
        return Err(OAuthError::InvalidClientId);
    }

    if oauth_res.expires < Utc::now() {
        return Err(OAuthError::ExpiredToken);
    }

    Ok(oauth_res.user)
}
