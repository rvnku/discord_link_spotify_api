pub use crate::oauth::errors::OAuthError;
use serde::Deserialize;
use uuid::Uuid;

pub mod client;
pub mod errors;
pub mod providers;

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[async_trait::async_trait]
pub trait OAuthProvider: Send + Sync {
    fn generate_url_with_state(&self, scopes: &[&str]) -> (String, String);
    async fn exchange_code_for_token(&self, code: &str) -> Result<Token, OAuthError>;
}

pub fn generate_state() -> String {
    Uuid::new_v4().to_string()
}

pub type OAuthResult<T> = Result<T, OAuthError>;
