use discord_link_spotify_api::config::Config;
use toasty_cli::{Config as ToastyConfig, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Config::from_env()?;
    let config = ToastyConfig::load()?;
    let db = toasty::Db::builder()
        .models(toasty::models!(discord_link_spotify_api::*))
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;
    Ok(())
}
