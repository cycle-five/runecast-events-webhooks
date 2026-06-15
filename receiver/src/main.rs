mod config;
mod handler;
mod relay;
mod sig;

use std::sync::Arc;

use axum::{
    http::{header, HeaderValue},
    middleware,
    response::Response,
    routing::{get, post},
    Router,
};

use crate::config::ReceiverConfig;
use crate::handler::{handle_discord_webhook, health, AppState};
use crate::relay::HttpRelay;

/// Discord's Webhook Events docs require a valid `Content-Type` on PING acks.
/// Our handlers return bare `StatusCode`s (empty body), for which axum sends no
/// `Content-Type` — unlike Flask's default (`text/html`) that Discord's sample
/// relies on. Stamp one on any response missing it. The body stays empty;
/// `Content-Type` is just the header Discord wants present.
async fn ensure_content_type(mut res: Response) -> Response {
    res.headers_mut()
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static("application/json"));
    res
}

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
        // Discord requires a valid Content-Type on PING acks; our bodyless
        // StatusCode responses don't get one from axum. Stamp it on every
        // response (outermost layer → covers 204 / 401 / 413 alike).
        .layer(middleware::map_response(ensure_content_type))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!("discord-webhook-receiver listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn adds_content_type_when_missing() {
        // A bare StatusCode response (exactly what the handlers return) carries
        // no Content-Type — this also documents the axum behavior that motivates
        // the middleware.
        let res = StatusCode::NO_CONTENT.into_response();
        assert!(
            res.headers().get(header::CONTENT_TYPE).is_none(),
            "axum sends no Content-Type for a bare StatusCode"
        );
        let res = ensure_content_type(res).await;
        assert_eq!(
            res.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
        assert_eq!(res.status(), StatusCode::NO_CONTENT, "status is unchanged");
    }

    #[tokio::test]
    async fn preserves_existing_content_type() {
        let mut res = StatusCode::OK.into_response();
        res.headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        let res = ensure_content_type(res).await;
        assert_eq!(
            res.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/plain",
            "must not clobber an existing Content-Type"
        );
    }
}
