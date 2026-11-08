use crate::{
    errors::AppError,
    handlers::auth::CallbackQuery,
    oauth::{OAuthProvider, generate_state},
    session::store::{AUTH_SERVICE_KEY, DISCORD_STATE_KEY, DISCORD_USER_ID_KEY},
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

pub async fn login(
    State(AppState { discord, .. }): State<AppState>,
    session: Session,
) -> Result<Response, AppError> {
    // Store unique state to session
    let state = generate_state();
    session
        .insert(DISCORD_STATE_KEY, &state)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?;

    // Generate url for oauth
    let url = (*discord).clone().generate_url(&vec!["identify"], &state);

    // Redirect
    Ok(Redirect::temporary(&url).into_response())
}

pub async fn callback(
    State(AppState { discord, .. }): State<AppState>,
    Query(CallbackQuery { code, state }): Query<CallbackQuery>,
    session: Session,
) -> Result<Response, AppError> {
    // Restore and check discord oauth state
    session
        .remove::<String>(DISCORD_STATE_KEY)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?
        .filter(|expected| expected == &state)
        .ok_or(AppError::InvalidState)?;

    // Exchange code for token
    let token = (*discord).clone().exchange_code_for_token(&code).await?;

    // Fetch user id via access token
    let user_id = (*discord).clone().fetch_user(&token.access_token).await?.id;

    // Store user id to session
    session
        .insert(DISCORD_USER_ID_KEY, user_id)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?;

    // Restore and check service value
    session
        .remove::<String>(AUTH_SERVICE_KEY)
        .await
        .map_err(|e| AppError::Session(e.to_string()))?
        .filter(|service| service == &"spotify")
        .ok_or(AppError::InvalidAuthService)?;

    // Redirect to spotify login
    Ok(Redirect::temporary("/auth/spotify/login").into_response())
}
