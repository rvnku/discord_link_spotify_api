use crate::{
    handlers::{api, auth},
    middleware::api_key::require_api_key,
    state::AppState,
};
use axum::{Router, middleware, routing::get};
use tower_sessions::{MemoryStore, SessionManagerLayer};

pub fn build_router(state: AppState, layer: SessionManagerLayer<MemoryStore>) -> Router {
    let api = Router::new()
        .route(
            "/api/spotify/access-token/{user_id}",
            get(api::spotify::access_token),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_api_key,
        ));

    let auth = Router::new()
        .route("/auth/spotify/connect", get(auth::spotify::connect))
        .route("/auth/discord/login", get(auth::discord::login))
        .route("/auth/discord/callback", get(auth::discord::callback))
        .route("/auth/spotify/login", get(auth::spotify::login))
        .route("/auth/spotify/callback", get(auth::spotify::callback));

    Router::new()
        .merge(api)
        .merge(auth)
        .layer(layer)
        .with_state(state)
}
