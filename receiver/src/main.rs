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
    // Default to `info` when RUST_LOG is unset — otherwise EnvFilter defaults to
    // ERROR-only and the boundary's auth-failure warnings would be invisible,
    // defeating the observability this service exists to provide.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cfg = ReceiverConfig::from_env()?;
    let relay = Arc::new(HttpRelay::new(
        cfg.backend_internal_url.clone(),
        cfg.internal_token.clone(),
    )?);
    let state = AppState {
        public_key: cfg.public_key.clone(),
        relay,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/discord/events", post(handle_discord_webhook))
        // Discord webhook bodies are tiny (~140 bytes); cap well below axum's
        // 2 MB default so an unauthenticated request can't force large
        // signature-verification work.
        .layer(axum::extract::DefaultBodyLimit::max(64 * 1024))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("discord-webhook-receiver listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
