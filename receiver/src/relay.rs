use anyhow::Context;
use async_trait::async_trait;
use discord_webhook_events::{DiscordEvent, RelayEnvelope};

/// Delivers a verified, typed Discord event onward. The receiver has already
/// acked Discord (204) before this runs, so implementations are best-effort.
///
/// `async-trait` (rather than native async-fn-in-trait) because `AppState`
/// stores the relay as `Arc<dyn EventRelay>`, and a trait with native async
/// methods is not yet dyn-compatible without boxing.
#[async_trait]
pub trait EventRelay: Send + Sync {
    async fn deliver(&self, application_id: &str, event: &DiscordEvent) -> anyhow::Result<()>;
}

/// Relays to the backend's private `POST /internal/discord-event`, guarded by
/// a shared token header. The endpoint is reachable only on the compose network.
pub struct HttpRelay {
    client: reqwest::Client,
    url: String,
    token: String,
}

impl HttpRelay {
    pub fn new(backend_internal_url: String, token: String) -> anyhow::Result<Self> {
        // A request timeout is essential: the relay runs in a spawned task after
        // we've already 204'd Discord, so a hung backend connection would
        // otherwise leak tasks indefinitely. Bound it so a stalled backend can't
        // pile up.
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .context("building relay HTTP client")?;
        Ok(Self {
            client,
            url: format!(
                "{}/internal/discord-event",
                backend_internal_url.trim_end_matches('/')
            ),
            token,
        })
    }
}

#[async_trait]
impl EventRelay for HttpRelay {
    async fn deliver(&self, application_id: &str, event: &DiscordEvent) -> anyhow::Result<()> {
        let envelope = RelayEnvelope {
            application_id: application_id.to_string(),
            event: event.clone(),
        };
        let resp = self
            .client
            .post(&self.url)
            .header("X-Internal-Token", &self.token)
            .json(&envelope)
            .send()
            .await
            .context("relay request failed")?;
        if !resp.status().is_success() {
            anyhow::bail!("relay returned status {}", resp.status());
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    pub struct MockRelay {
        pub delivered: Mutex<Vec<(String, DiscordEvent)>>,
    }

    #[async_trait]
    impl EventRelay for MockRelay {
        async fn deliver(&self, application_id: &str, event: &DiscordEvent) -> anyhow::Result<()> {
            self.delivered
                .lock()
                .unwrap()
                .push((application_id.to_string(), event.clone()));
            Ok(())
        }
    }

    /// A relay that always fails — proves best-effort semantics: the handler
    /// must still 204 when delivery errors.
    pub struct FailingRelay;

    #[async_trait]
    impl EventRelay for FailingRelay {
        async fn deliver(
            &self,
            _application_id: &str,
            _event: &DiscordEvent,
        ) -> anyhow::Result<()> {
            anyhow::bail!("relay unavailable")
        }
    }

    #[tokio::test]
    async fn mock_records_delivery() {
        use discord_webhook_events::{ApplicationEventData, PartialUser};
        let m = MockRelay::default();
        let ev = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
            integration_type: 1,
            scopes: vec![],
            user: PartialUser {
                id: "1".into(),
                username: "u".into(),
                global_name: None,
                avatar: None,
            },
            guild: None,
        });
        m.deliver("app1", &ev).await.unwrap();
        assert_eq!(m.delivered.lock().unwrap().len(), 1);
    }
}
