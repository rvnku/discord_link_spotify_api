use crate::oauth::errors::OAuthError;
use serde::Deserialize;
use uuid::Uuid;

pub mod discord;
pub mod errors;
pub mod spotify;

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

#[async_trait::async_trait]
pub trait OAuthProvider: Send + Sync {
    fn generate_url(&self, scopes: &[&str], state: &str) -> String;
    async fn exchange_code_for_token(&self, code: &str) -> Result<Token, OAuthError>;
}

pub fn generate_state() -> String {
    Uuid::new_v4().to_string()
}
