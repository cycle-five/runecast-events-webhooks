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
//! let event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
//!     application_id: "123".to_string(),
//!     user_id: "456".to_string(),
//!     guild_id: Some("789".to_string()),
//! });
//!
//! let json = serde_json::to_string(&event).unwrap();
//! ```

mod events;

pub use events::{
    ApplicationEventData, DiscordEvent, EntitlementEventData, GameMessageEventData,
    LobbyMessageEventData, QuestEventData,
};
