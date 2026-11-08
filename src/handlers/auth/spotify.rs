use crate::{
    AppError, AppResponse, AppState, database,
    handlers::auth::CallbackQuery,
    oauth::OAuthProvider,
    session::{
        ext::SessionExt,
        store::{AUTH_SERVICE_KEY, DISCORD_USER_ID_KEY, SPOTIFY_STATE_KEY},
    },
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn connect(session: Session) -> AppResponse {
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
) -> AppResponse {
    // Generate url for oauth
    let (url, state) = (*spotify)
        .clone()
        .generate_url_with_state(&vec!["streaming"]);
    session.set(SPOTIFY_STATE_KEY, &state).await?;

    // Redirect
    Ok(Redirect::temporary(&url).into_response())
}

pub async fn callback(
    State(AppState { db, spotify, .. }): State<AppState>,
    Query(CallbackQuery { code, state }): Query<CallbackQuery>,
    session: Session,
) -> AppResponse {
    // Restore and check spotify oauth state
    session
        .take_matching(SPOTIFY_STATE_KEY, &state, AppError::InvalidState)
        .await?;

    // Exchange code for token
    let token = (*spotify).clone().exchange_code_for_token(&code).await?;

    // Restore user id from session
    let user_id = session
        .take::<String>(DISCORD_USER_ID_KEY)
        .await?
        .ok_or(AppError::MissingUserId)?;

    // Upsert tokens to database
    let mut db = db.clone();
    database::upsert(&mut db, user_id, token).await?;

    // Success page
    Ok("Success authorized".into_response())
}
