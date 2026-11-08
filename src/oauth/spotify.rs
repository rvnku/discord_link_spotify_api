use crate::oauth::{OAuthProvider, Token, errors::OAuthError};
use reqwest::Client;
use url::Url;

const AUTHORIZE_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

#[derive(Debug, Clone)]
pub struct SpotifyOAuth {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    client: Client,
}

impl SpotifyOAuth {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            client: Client::new(),
        }
    }
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<Token, OAuthError> {
        let response = self
            .client
            .post(TOKEN_URL)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await
            .map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))?;

        if !status.is_success() {
            return Err(OAuthError::TokenRequestFailed(format!(
                "spotify returned {status}",
            )));
        }

        serde_json::from_str(&body).map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))
    }
}

#[async_trait::async_trait]
impl OAuthProvider for SpotifyOAuth {
    fn generate_url(&self, scopes: &[&str], state: &str) -> String {
        let mut url = Url::parse(AUTHORIZE_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &scopes.join(" "))
            .append_pair("state", state);
        url.into()
    }
    async fn exchange_code_for_token(&self, code: &str) -> Result<Token, OAuthError> {
        let response = self
            .client
            .post(TOKEN_URL)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))?;

        if !status.is_success() {
            return Err(OAuthError::TokenRequestFailed(format!(
                "spotify returned {status}"
            )));
        }

        serde_json::from_str(&body).map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))
    }
}
