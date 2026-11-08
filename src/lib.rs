pub use crate::errors::AppError;
pub use crate::state::AppState;
use axum::response::Response;

pub mod config;
pub mod database;
pub mod errors;
pub mod events;
pub mod handlers;
pub mod middleware;
pub mod oauth;
pub mod router;
pub mod session;
pub mod state;

pub type AppResult<T> = Result<T, AppError>;
pub type AppResponse = AppResult<Response>;
