use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    // Server
    pub bind_addr: String,
    pub base_url: String,
    pub api_key: String,

    // Discord
    pub discord_client_id: String,
    pub discord_client_secret: String,

    // Spotify
    pub spotify_client_id: String,
    pub spotify_client_secret: String,

    // Database
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let filename = match std::env::var("APP_ENV").ok().as_deref() {
            Some("prod" | "production") => ".env.prod",
            Some("dev" | "develop") => ".env.dev",
            Some(other) => bail!(
                "invalid value of environment variable app_key: {other:?}, expected one of: production, develop"
            ),
            None => ".env",
        };

        dotenvy::from_filename(filename).ok();
        envy::from_env().context("missing or invalid environment variables")
    }
}
