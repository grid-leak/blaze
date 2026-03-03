use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Discord API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Discord rejected the access token (HTTP {0})")]
    InvalidToken(u16),
}

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
}

pub async fn fetch_discord_user(access_token: &str) -> Result<DiscordUser, OAuthError> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(OAuthError::InvalidToken(response.status().as_u16()));
    }

    Ok(response.json::<DiscordUser>().await?)
}
