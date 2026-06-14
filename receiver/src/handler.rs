use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use discord_webhook_events::{DiscordEvent, DiscordWebhookPayload};

use crate::relay::EventRelay;
use crate::sig::{timestamp_within_window, verify_discord_signature};

const REPLAY_WINDOW_SECS: i64 = 300;
/// Cap raw-body diagnostic logging so a large body can't flood logs.
const RAW_LOG_CAP: usize = 2048;

#[derive(Clone)]
pub struct AppState {
    pub public_key: String,
    pub relay: Arc<dyn EventRelay>,
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn handle_discord_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let signature = match headers
        .get("X-Signature-Ed25519")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED,
    };
    let timestamp = match headers
        .get("X-Signature-Timestamp")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED,
    };
    if !timestamp_within_window(timestamp, REPLAY_WINDOW_SECS) {
        tracing::warn!("Discord webhook timestamp outside replay window");
        return StatusCode::UNAUTHORIZED;
    }
    if !verify_discord_signature(&state.public_key, signature, timestamp, &body) {
        tracing::warn!("Discord webhook signature verification failed");
        return StatusCode::UNAUTHORIZED;
    }

    // Parse — ACK-RESILIENT. A signed-but-unparseable body must NOT 400
    // (that triggers Discord retry storms). Log a bounded raw body so the
    // payload can be captured to fix the parser, then 204.
    let payload: DiscordWebhookPayload = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            // Slice the BYTES (byte-safe) before lossy-converting — slicing a
            // &str at RAW_LOG_CAP could land mid-UTF-8-char and panic.
            let capped = &body[..body.len().min(RAW_LOG_CAP)];
            tracing::error!(error = %e, raw_body = %String::from_utf8_lossy(capped), "Unparseable (but signed) Discord webhook; acking");
            return StatusCode::NO_CONTENT;
        }
    };

    if payload.kind == 0 {
        return StatusCode::NO_CONTENT; // PING
    }

    if let Some(event_body) = payload.event {
        // Best-effort relay; we've already decided to ack Discord.
        if matches!(
            &event_body.event,
            DiscordEvent::EntitlementCreate(_)
                | DiscordEvent::EntitlementUpdate(_)
                | DiscordEvent::EntitlementDelete(_)
                | DiscordEvent::ApplicationAuthorized(_)
                | DiscordEvent::ApplicationDeauthorized(_)
        ) {
            if let Err(e) = state
                .relay
                .deliver(&payload.application_id, &event_body.event)
                .await
            {
                tracing::error!(error = %e, "Failed to relay Discord event to backend");
            }
        } else {
            tracing::debug!(
                kind = event_body.event.event_type(),
                "Unhandled Discord event; ignoring"
            );
        }
    }
    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relay::tests::MockRelay;
    use axum::http::HeaderValue;
    use chrono::Utc;
    use ed25519_dalek::{Signer, SigningKey};

    const SEED: [u8; 32] = [
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x7f, 0x60,
    ];

    fn signed_state_and_headers(body: &[u8]) -> (AppState, HeaderMap, Arc<MockRelay>) {
        let key = SigningKey::from_bytes(&SEED);
        let pk = hex::encode(key.verifying_key().to_bytes());
        let ts = Utc::now().timestamp().to_string();
        let mut msg = ts.as_bytes().to_vec();
        msg.extend_from_slice(body);
        let sig = hex::encode(key.sign(&msg).to_bytes());
        let mut headers = HeaderMap::new();
        headers.insert("X-Signature-Ed25519", HeaderValue::from_str(&sig).unwrap());
        headers.insert("X-Signature-Timestamp", HeaderValue::from_str(&ts).unwrap());
        let relay = Arc::new(MockRelay::default());
        (
            AppState {
                public_key: pk,
                relay: relay.clone(),
            },
            headers,
            relay,
        )
    }

    #[tokio::test]
    async fn entitlement_create_relays_and_204s() {
        let body = br#"{"version":1,"application_id":"app1","type":1,"event":{"type":"ENTITLEMENT_CREATE","timestamp":"2026-06-01T20:00:00Z","data":{"id":"1","sku_id":"2","application_id":"app1","user_id":"3","type":1,"deleted":false}}}"#;
        let (state, headers, relay) = signed_state_and_headers(body);
        let code = handle_discord_webhook(State(state), headers, Bytes::from(body.to_vec())).await;
        assert_eq!(code, StatusCode::NO_CONTENT);
        assert_eq!(relay.delivered.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn ping_204s_without_relay() {
        let body = br#"{"version":1,"application_id":"app1","type":0}"#;
        let (state, headers, relay) = signed_state_and_headers(body);
        let code = handle_discord_webhook(State(state), headers, Bytes::from(body.to_vec())).await;
        assert_eq!(code, StatusCode::NO_CONTENT);
        assert_eq!(relay.delivered.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn unparseable_but_signed_204s_and_does_not_relay() {
        let body =
            br#"{"version":1,"application_id":"app1","type":1,"event":{"type":"ENTITLEMENT_CR"#; // truncated
        let (state, headers, relay) = signed_state_and_headers(body);
        let code = handle_discord_webhook(State(state), headers, Bytes::from(body.to_vec())).await;
        assert_eq!(code, StatusCode::NO_CONTENT, "ack-resilience: never 400");
        assert_eq!(relay.delivered.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn bad_signature_401s() {
        let body = br#"{"version":1,"application_id":"app1","type":0}"#;
        let (mut state, headers, _relay) = signed_state_and_headers(body);
        // Corrupt the public key so verification fails.
        state.public_key = "00".repeat(32);
        let code = handle_discord_webhook(State(state), headers, Bytes::from(body.to_vec())).await;
        assert_eq!(code, StatusCode::UNAUTHORIZED);
    }
}
