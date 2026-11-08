use crate::{
    AppError, AppResult, AppState,
    database::{self, repository},
};
use axum::{
    Json,
    extract::{Path, State},
};
use jiff::{Span, Timestamp};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
}

pub async fn access_token(
    Path(user_id): Path<String>,
    State(AppState { db, spotify, .. }): State<AppState>,
) -> AppResult<Json<AccessTokenResponse>> {
    let mut db = db.clone();
    let token = repository::find_by_user_id(&mut db, &user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::TokenNotFound)?;

    if token.expires_at > Timestamp::now() + Span::new().seconds(60) {
        return Ok(Json(AccessTokenResponse {
            access_token: token.access_token,
        }));
    }

    let token = spotify.refresh_token(&token.refresh_token).await?;
    let access_token = token.access_token.clone();

    database::upsert(&mut db, user_id, token).await?;

    Ok(Json(AccessTokenResponse {
        access_token: access_token,
    }))
}
