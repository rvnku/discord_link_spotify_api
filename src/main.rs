use anyhow::{Context, Result};
use discord_link_spotify_api::{
    config::Config, events::wait_for_signal, router::build_router, state::AppState,
};
use tokio::net::TcpListener;
use tower_sessions::{
    Expiry, MemoryStore, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tracing_subscriber::{
    EnvFilter, fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<()> {
    registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(layer())
        .init();

    let config = Config::from_env().context("failed to load application configuration")?;
    let state = AppState::from_config(config.clone())
        .await
        .context("failed to build application state")?;
    let layer = SessionManagerLayer::new(MemoryStore::default())
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)))
        .with_same_site(SameSite::Lax);
    let router = build_router(state, layer);
    let listener = TcpListener::bind(&config.bind_addr)
        .await
        .with_context(|| format!("failed to bind TCP listener to {}", &config.bind_addr))?;

    tracing::info!("Listening on {}", &config.bind_addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    wait_for_signal().await;
    tracing::info!("Shutdown signal received");
}
