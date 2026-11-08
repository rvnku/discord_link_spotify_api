use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("token request failed: {0}")]
    TokenRequestFailed(String),

    #[error("user request failed: {0}")]
    UserRequestFailed(String),

    #[error("{provider}: missing required field: {field}")]
    MissingRequiredField { provider: String, field: String },
}
