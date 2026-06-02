# runecast-events-webhooks

A Rust library for handling Discord webhook events with type-safe event definitions.

## Overview

This library provides comprehensive type definitions and utilities for working with Discord webhook events. It supports all Discord event types including:

- **Application Events**: Authorization and deauthorization
- **Entitlement Events**: Create, update, and delete operations
- **Quest Events**: User enrollment tracking
- **Game Events**: Direct messages and lobby messages (create, update, delete)

## Features

- 🔒 Type-safe event handling with Rust's strong type system
- 📦 Serde-based JSON serialization/deserialization
- ✅ Comprehensive test coverage
- 📚 Well-documented API with examples

## Supported Events

### Applications
- `APPLICATION_AUTHORIZED`
- `APPLICATION_DEAUTHORIZED`

### Entitlements
- `ENTITLEMENT_CREATE`
- `ENTITLEMENT_UPDATE`
- `ENTITLEMENT_DELETE`

### Quests
- `QUEST_USER_ENROLLMENT`

### Games
- `GAME_DIRECT_MESSAGE_CREATE`
- `GAME_DIRECT_MESSAGE_UPDATE`
- `GAME_DIRECT_MESSAGE_DELETE`
- `LOBBY_MESSAGE_CREATE`
- `LOBBY_MESSAGE_UPDATE`
- `LOBBY_MESSAGE_DELETE`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
runecast-events-webhooks = "0.2.0"
```

### Basic Example

```rust
use runecast_events_webhooks::{DiscordEvent, ApplicationEventData};

// Create an event
let event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
    application_id: "123456789".to_string(),
    user_id: "987654321".to_string(),
    guild_id: Some("555555555".to_string()),
});

// Serialize to JSON
let json = serde_json::to_string(&event).unwrap();

// Deserialize from JSON
let parsed: DiscordEvent = serde_json::from_str(&json).unwrap();

// Get event type
println!("Event type: {}", event.event_type());
```

### Running Examples

```bash
cargo run --example basic_usage
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

## License

This project is available under the MIT or Apache 2.0 license, at your option.
