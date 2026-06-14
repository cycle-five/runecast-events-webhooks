use anyhow::Context;
use async_trait::async_trait;
use discord_webhook_events::{DiscordEvent, RelayEnvelope};

/// Delivers a verified, typed Discord event onward. The receiver has already
/// acked Discord (204) before this runs, so implementations are best-effort.
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
    pub fn new(backend_internal_url: String, token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: format!(
                "{}/internal/discord-event",
                backend_internal_url.trim_end_matches('/')
            ),
            token,
        }
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
