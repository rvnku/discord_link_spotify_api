use crate::{
    database::repository,
    errors::AppError,
    handlers::auth::CallbackQuery,
    oauth::{OAuthProvider, generate_state},
    session::store::{AUTH_SERVICE_KEY, DISCORD_USER_ID_KEY, SPOTIFY_STATE_KEY},
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use jiff::{Span, Timestamp};
use tower_sessions::Session;

pub async fn connect(session: Session) -> Result<Response, AppError> {
    // Store service key to session
    session
        .insert(AUTH_SERVICE_KEY, "spotify")
        .await
        .map_err(|e| AppError::Session(e.to_string()))?;

    // Redirect to discord login
    Ok(Redirect::temporary("/auth/discord/login").into_response())
}

pub async fn login(
    State(AppState { spotify, .. }): State<AppState>,
    session: Session,
) -> Result<Response, AppError> {
    // Store unique state to session
    let state = generate_state();
    session
        .insert(SPOTIFY_STATE_KEY, &state)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?;

    // Generate url for oauth
    let url = (*spotify).clone().generate_url(&vec!["streaming"], &state);

    // Redirect
    Ok(Redirect::temporary(&url).into_response())
}

pub async fn callback(
    State(AppState { db, spotify, .. }): State<AppState>,
    Query(CallbackQuery { code, state }): Query<CallbackQuery>,
    session: Session,
) -> Result<Response, AppError> {
    // Restore and check spotify oauth state
    session
        .remove::<String>(SPOTIFY_STATE_KEY)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?
        .filter(|expected| expected == &state)
        .ok_or(AppError::InvalidState)?;

    // Exchange code for token
    let token = (*spotify).clone().exchange_code_for_token(&code).await?;
    let access_token = token.access_token;
    let refresh_token = token
        .refresh_token
        .ok_or_else(|| AppError::Internal("spotify: refresh_token is missing".into()))?;
    let expires_in = token
        .expires_in
        .ok_or_else(|| AppError::Internal("spotify: expires_in is missing".into()))?;
    let expires_at = Timestamp::now() + Span::new().seconds(expires_in as i64);

    // Restore user id from session
    let user_id = session
        .remove::<String>(DISCORD_USER_ID_KEY)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?
        .ok_or(AppError::MissingUserId)?;

    // Upsert tokens to database
    let mut db = db.clone();
    repository::upsert(
        &mut db,
        user_id,
        access_token.into(),
        refresh_token.into(),
        expires_at,
    )
    .await
    .map_err(|e| AppError::Database(e))?;

    // Success page
    Ok("Success authorized".into_response())
}
