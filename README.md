# discord-webhook-events

A Rust library providing type-safe definitions for [Discord Webhook Events](https://discord.com/developers/docs/events/webhook-events).

## Overview

When you register a Webhook Events URL in the Discord Developer Portal, Discord POSTs a `DiscordWebhookPayload` to your endpoint for each subscribed event. This crate gives you the typed structs to deserialize those payloads — no `serde_json::Value` unwrapping required.

Supported event categories:

- **Application**: `APPLICATION_AUTHORIZED`, `APPLICATION_DEAUTHORIZED`
- **Entitlements**: `ENTITLEMENT_CREATE`, `ENTITLEMENT_UPDATE`, `ENTITLEMENT_DELETE`
- **Quests**: `QUEST_USER_ENROLLMENT`
- **Game messages**: `GAME_DIRECT_MESSAGE_CREATE/UPDATE/DELETE`, `LOBBY_MESSAGE_CREATE/UPDATE/DELETE`

## Installation

```toml
[dependencies]
discord-webhook-events = "0.3"
```

## Usage

### Receiving a webhook

The outer type is `DiscordWebhookPayload`. Deserialize the raw POST body into it, then match on `payload.event`:

```rust
use discord_webhook_events::{DiscordEvent, DiscordWebhookPayload};

fn handle_webhook(body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let payload: DiscordWebhookPayload = serde_json::from_str(body)?;

    // kind=0 is a PING — Discord sends this once when you register the URL.
    // Respond with 204 No Content.
    if payload.kind == 0 {
        return Ok(());
    }

    match payload.event.as_ref().map(|b| &b.event) {
        Some(DiscordEvent::EntitlementCreate(ent)) => {
            println!("New entitlement: sku={} user={}", ent.sku_id, ent.user_id);
        }
        Some(DiscordEvent::EntitlementDelete(ent)) => {
            println!("Entitlement revoked: sku={} user={}", ent.sku_id, ent.user_id);
        }
        Some(DiscordEvent::ApplicationAuthorized(app)) => {
            // application_id lives in the outer envelope, not in app data
            println!(
                "App {} authorized by user {} (scopes: {})",
                payload.application_id,
                app.user.id,
                app.scopes.join(", ")
            );
        }
        Some(other) => {
            println!("Unhandled event: {}", other.event_type());
        }
        None => {}
    }
    Ok(())
}
```

### Wire shape

Discord sends a JSON body like this for a `kind=1` (event) payload:

```json
{
  "version": 1,
  "application_id": "1234560123453231555",
  "type": 1,
  "event": {
    "type": "ENTITLEMENT_CREATE",
    "timestamp": "2026-06-01T20:00:00Z",
    "data": {
      "id": "1100000000000000001",
      "sku_id": "2200000000000000002",
      "application_id": "1234560123453231555",
      "user_id": "3300000000000000003",
      "type": 1,
      "deleted": false
    }
  }
}
```

`DiscordEvent` uses adjacent tagging (`#[serde(tag = "type", content = "data")]`) matching this exact shape, combined with `#[serde(flatten)]` on `DiscordEventBody.event` — so the entire inner payload deserializes into a typed enum without an intermediate `Value` step.

## Running examples

```bash
cargo run --example basic_usage
```

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
