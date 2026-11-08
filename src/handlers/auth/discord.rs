use crate::{
    AppError, AppResponse, AppState,
    handlers::auth::CallbackQuery,
    oauth::OAuthProvider,
    session::{
        ext::SessionExt,
        store::{AUTH_SERVICE_KEY, DISCORD_STATE_KEY, DISCORD_USER_ID_KEY},
    },
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn login(
    State(AppState { discord, .. }): State<AppState>,
    session: Session,
) -> AppResponse {
    // Generate url for oauth
    let (url, state) = (*discord)
        .clone()
        .generate_url_with_state(&vec!["identify"]);
    session.set(DISCORD_STATE_KEY, &state).await?;

    // Redirect
    Ok(Redirect::temporary(&url).into_response())
}

pub async fn callback(
    State(AppState { discord, .. }): State<AppState>,
    Query(CallbackQuery { code, state }): Query<CallbackQuery>,
    session: Session,
) -> AppResponse {
    // Restore and check discord oauth state
    session
        .take_matching(DISCORD_STATE_KEY, &state, AppError::InvalidState)
        .await?;

    // Exchange code for token
    let token = (*discord).clone().exchange_code_for_token(&code).await?;

    // Fetch user id via access token
    let user_id = (*discord).clone().fetch_user(&token.access_token).await?.id;

    // Store user id to session
    session.set(DISCORD_USER_ID_KEY, user_id).await?;

    // Restore and check service value
    session
        .take_matching(AUTH_SERVICE_KEY, "spotify", AppError::InvalidAuthService)
        .await?;

    // Redirect to spotify login
    Ok(Redirect::temporary("/auth/spotify/login").into_response())
}
