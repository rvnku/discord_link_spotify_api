use crate::oauth::{OAuthError, OAuthResult, Token};
use reqwest::Client;
use serde_json::Value;
use std::fmt::{Display, Formatter, Result};
use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub enum Provider {
    Discord,
    Spotify,
}
impl Display for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Discord => "discord",
            Self::Spotify => "spotify",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct OAuthClient {
    provider: Provider,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    client: Client,
}

impl OAuthClient {
    pub fn new(
        provider: Provider,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            provider,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            client: Client::new(),
        }
    }

    pub fn http(&self) -> &Client {
        &self.client
    }

    pub fn build_authorize_url(&self, authorize_url: &str, scopes: &[&str], state: &str) -> String {
        let mut url = Url::parse(authorize_url).expect("valid authorize url");
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &scopes.join(" "))
            .append_pair("state", state);
        match self.provider {
            Provider::Discord => url.query_pairs_mut().append_pair("prompt", "consent"),
            Provider::Spotify => url.query_pairs_mut().append_pair("show_dialog", "true"),
        };
        url.into()
    }

    pub async fn exchange_code_for_token(&self, token_url: &str, code: &str) -> OAuthResult<Token> {
        let mut form = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];
        if self.provider == Provider::Discord {
            form.push(("client_id", self.client_id.as_str()));
            form.push(("client_secret", self.client_secret.as_str()));
        }
        self.post_token(token_url, &form).await
    }

    pub async fn refresh_token(&self, token_url: &str, refresh_token: &str) -> OAuthResult<Token> {
        let mut form = vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        if self.provider == Provider::Discord {
            form.push(("client_id", self.client_id.as_str()));
            form.push(("client_secret", self.client_secret.as_str()));
        }
        self.post_token(token_url, &form).await
    }

    async fn post_token(&self, token_url: &str, form: &[(&str, &str)]) -> OAuthResult<Token> {
        let mut request = self.client.post(token_url).form(form);
        if self.provider == Provider::Spotify {
            request = request.basic_auth(&self.client_id, Some(&self.client_secret));
        }
        let response = request
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
                "{} returned {status}",
                self.provider
            )));
        }
        let token: Value = serde_json::from_str(&body)
            .map_err(|e| OAuthError::TokenRequestFailed(e.to_string()))?;

        let provider = match self.provider {
            Provider::Discord => "discord",
            Provider::Spotify => "spotify",
        };
        let field = |name: &str| OAuthError::MissingRequiredField {
            provider: provider.to_string(),
            field: name.into(),
        };

        Ok(Token {
            access_token: token["access_token"]
                .as_str()
                .ok_or_else(|| field("access_token"))?
                .to_string(),
            refresh_token: token["refresh_token"]
                .as_str()
                .ok_or_else(|| field("refresh_token"))?
                .to_string(),
            expires_in: token["expires_in"]
                .as_i64()
                .ok_or_else(|| field("expires_in"))?,
        })
    }
}
