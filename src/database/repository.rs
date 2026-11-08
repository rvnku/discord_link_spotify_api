use crate::database::models::SpotifyToken;
use jiff::Timestamp;
use toasty::{Db, Result};

pub async fn find_by_user_id(db: &mut Db, user_id: &str) -> Result<Option<SpotifyToken>> {
    SpotifyToken::filter_by_user_id(user_id)
        .first()
        .exec(db)
        .await
}

pub async fn upsert(
    db: &mut Db,
    user_id: String,
    access_token: String,
    refresh_token: String,
    expires_at: Timestamp,
) -> Result<SpotifyToken> {
    SpotifyToken::upsert_by_user_id(user_id)
        .access_token(access_token)
        .refresh_token(refresh_token)
        .expires_at(expires_at)
        .exec(db)
        .await
}
