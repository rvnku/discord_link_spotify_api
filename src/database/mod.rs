use crate::{AppError, AppResult, database::models::SpotifyToken, oauth::Token};
use jiff::{Span, Timestamp};
use toasty::Db;

pub mod models;
pub mod repository;

pub async fn upsert(db: &mut Db, user_id: String, token: Token) -> AppResult<SpotifyToken> {
    let expires_at = Timestamp::now() + Span::new().seconds(token.expires_in);
    repository::upsert(
        db,
        user_id,
        token.access_token,
        token.refresh_token,
        expires_at,
    )
    .await
    .map_err(AppError::Database)
}
