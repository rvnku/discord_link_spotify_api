use crate::oauth::{
    OAuthError, OAuthProvider, OAuthResult, Token,
    client::{OAuthClient, Provider},
    generate_state,
};
use serde::Deserialize;

const AUTHORIZE_URL: &str = "https://discord.com/oauth2/authorize";
const TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const USER_URL: &str = "https://discord.com/api/v10/users/@me";

#[derive(Debug, Clone)]
pub struct DiscordOAuth(OAuthClient);

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub id: String,
}

impl DiscordOAuth {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self(OAuthClient::new(
            Provider::Discord,
            client_id,
            client_secret,
            redirect_uri,
        ))
    }

    pub async fn fetch_user(&self, access_token: &str) -> OAuthResult<DiscordUser> {
        self.0
            .http()
            .get(USER_URL)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| OAuthError::UserRequestFailed(e.to_string()))?
            .json()
            .await
            .map_err(|e| OAuthError::UserRequestFailed(e.to_string()))
    }
}

#[async_trait::async_trait]
impl OAuthProvider for DiscordOAuth {
    fn generate_url_with_state(&self, scopes: &[&str]) -> (String, String) {
        let state = generate_state();
        (
            self.0.build_authorize_url(AUTHORIZE_URL, scopes, &state),
            state,
        )
    }
    async fn exchange_code_for_token(&self, code: &str) -> OAuthResult<Token> {
        self.0.exchange_code_for_token(TOKEN_URL, code).await
    }
}
