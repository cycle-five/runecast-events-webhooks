mod config;
mod handler;
mod relay;
mod sig;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::config::ReceiverConfig;
use crate::handler::{handle_discord_webhook, health, AppState};
use crate::relay::HttpRelay;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = ReceiverConfig::from_env()?;
    let relay = Arc::new(HttpRelay::new(
        cfg.backend_internal_url.clone(),
        cfg.internal_token.clone(),
    ));
    let state = AppState {
        public_key: cfg.public_key.clone(),
        relay,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/discord/events", post(handle_discord_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("discord-webhook-receiver listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
