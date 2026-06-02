//! # Runecast Events Webhooks
//!
//! This library provides type definitions and utilities for handling Discord webhook events.
//! It supports all Discord event types including Application, Entitlement, Quest, and Game events.
//!
//! ## Example
//!
//! ```rust
//! use runecast_events_webhooks::{DiscordEvent, ApplicationEventData};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
//!     application_id: "123".to_string(),
//!     user_id: "456".to_string(),
//!     guild_id: Some("789".to_string()),
//! });
//!
//! // Proper error handling with ? operator
//! let json = serde_json::to_string(&event)?;
//! let parsed: DiscordEvent = serde_json::from_str(&json)?;
//! # Ok(())
//! # }
//! ```

mod events;

pub use events::{
    ApplicationEventData, DiscordEvent, DiscordEventBody, DiscordWebhookPayload,
    EntitlementEventData, GameMessageEventData, LobbyMessageEventData, QuestEventData,
    // Export event type constants for consistency
    APPLICATION_AUTHORIZED, APPLICATION_DEAUTHORIZED,
    ENTITLEMENT_CREATE, ENTITLEMENT_UPDATE, ENTITLEMENT_DELETE,
    QUEST_USER_ENROLLMENT,
    GAME_DIRECT_MESSAGE_CREATE, GAME_DIRECT_MESSAGE_UPDATE, GAME_DIRECT_MESSAGE_DELETE,
    LOBBY_MESSAGE_CREATE, LOBBY_MESSAGE_UPDATE, LOBBY_MESSAGE_DELETE,
};
