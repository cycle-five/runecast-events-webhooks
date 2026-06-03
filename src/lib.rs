//! # Runecast Events Webhooks
//!
//! This library provides type definitions and utilities for handling Discord webhook events.
//! It supports all Discord event types including Application, Entitlement, Quest, and Game events.
//!
//! ## Example
//!
//! ```rust
//! use discord_webhook_events::{DiscordWebhookPayload, DiscordEvent};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Deserialize the raw POST body Discord sends to your webhook URL.
//! let raw = r#"{
//!     "version": 1,
//!     "application_id": "1234560123453231555",
//!     "type": 1,
//!     "event": {
//!         "type": "ENTITLEMENT_CREATE",
//!         "timestamp": "2026-06-01T20:00:00Z",
//!         "data": {
//!             "id": "1100000000000000001",
//!             "sku_id": "2200000000000000002",
//!             "application_id": "1234560123453231555",
//!             "user_id": "3300000000000000003",
//!             "type": 1,
//!             "deleted": false
//!         }
//!     }
//! }"#;
//!
//! let payload: DiscordWebhookPayload = serde_json::from_str(raw)?;
//! match payload.event.as_ref().map(|b| &b.event) {
//!     Some(DiscordEvent::EntitlementCreate(ent)) => {
//!         println!("New entitlement for user {}", ent.user_id);
//!     }
//!     None => { /* PING — ack with 204 */ }
//!     _ => {}
//! }
//! # Ok(())
//! # }
//! ```

mod events;

pub use events::{
    ApplicationEventData, DiscordEvent, DiscordEventBody, DiscordWebhookPayload,
    EntitlementEventData, GameMessageEventData, LobbyMessageEventData, PartialGuild,
    PartialUser, QuestEventData,
    // Export event type constants for consistency
    APPLICATION_AUTHORIZED, APPLICATION_DEAUTHORIZED,
    ENTITLEMENT_CREATE, ENTITLEMENT_UPDATE, ENTITLEMENT_DELETE,
    QUEST_USER_ENROLLMENT,
    GAME_DIRECT_MESSAGE_CREATE, GAME_DIRECT_MESSAGE_UPDATE, GAME_DIRECT_MESSAGE_DELETE,
    LOBBY_MESSAGE_CREATE, LOBBY_MESSAGE_UPDATE, LOBBY_MESSAGE_DELETE,
};
