use crate::oauth::{
    OAuthProvider, OAuthResult, Token,
    client::{OAuthClient, Provider},
    generate_state,
};

const AUTHORIZE_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

#[derive(Debug, Clone)]
pub struct SpotifyOAuth(OAuthClient);

impl SpotifyOAuth {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self(OAuthClient::new(
            Provider::Spotify,
            client_id,
            client_secret,
            redirect_uri,
        ))
    }
    pub async fn refresh_token(&self, refresh_token: &str) -> OAuthResult<Token> {
        self.0.refresh_token(TOKEN_URL, refresh_token).await
    }
}

#[async_trait::async_trait]
impl OAuthProvider for SpotifyOAuth {
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
