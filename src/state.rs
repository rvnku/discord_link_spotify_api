use crate::{
    config::Config,
    oauth::providers::{discord::DiscordOAuth, spotify::SpotifyOAuth},
};
use anyhow::{Context, Result};
use std::sync::Arc;
use toasty::{Db, models};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub api_key: String,
    pub discord: Arc<DiscordOAuth>,
    pub spotify: Arc<SpotifyOAuth>,
}

impl AppState {
    pub async fn from_config(config: Config) -> Result<Self> {
        let db = Db::builder()
            .models(models!(crate::*))
            .connect(&config.database_url)
            .await
            .context("failet to connect to database")?;

        Ok(Self {
            db,
            api_key: config.api_key,
            discord: Arc::new(DiscordOAuth::new(
                config.discord_client_id,
                config.discord_client_secret,
                format!("{}/auth/discord/callback", config.base_url),
            )),
            spotify: Arc::new(SpotifyOAuth::new(
                config.spotify_client_id,
                config.spotify_client_secret,
                format!("{}/auth/spotify/callback", config.base_url),
            )),
        })
    }
}
