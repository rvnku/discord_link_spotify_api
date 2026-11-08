use serde::Deserialize;

#[derive(Deserialize)]
pub enum OAuthError {
    TokenRequestFailed(String),
    UserRequestFailed(String),
}
