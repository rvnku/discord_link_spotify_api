use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::oauth::errors::OAuthError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] toasty::Error),

    #[error("session error: {0}")]
    Session(String),

    #[error("missing or invalid session state")]
    InvalidState,

    #[error("missing user_id in session")]
    MissingUserId,

    #[error("auth service marker missing from session")]
    InvalidAuthService,

    #[error("token record not found for user")]
    TokenNotFound,

    #[error("internal error: {0}")]
    Internal(String),
}

impl From<OAuthError> for AppError {
    fn from(value: OAuthError) -> Self {
        match value {
            OAuthError::TokenRequestFailed(error) => AppError::Internal(error),
            OAuthError::UserRequestFailed(error) => AppError::Internal(error),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            AppError::Database(_) | AppError::Session(_) | AppError::Internal(_) => {
                tracing::error!(error = %self, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            AppError::InvalidState | AppError::MissingUserId | AppError::InvalidAuthService => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            AppError::TokenNotFound => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
        }
    }
}
