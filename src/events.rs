use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Event type constants to ensure consistency between serde rename and event_type method.
// Note: These constants must match the string literals in #[serde(rename = "...")] attributes below.
// Serde's rename attribute requires string literals and cannot use const references,
// so the strings are duplicated by necessity. The constants are used in the event_type() method
// to provide a single source of truth for runtime usage.
pub const APPLICATION_AUTHORIZED: &str = "APPLICATION_AUTHORIZED";
pub const APPLICATION_DEAUTHORIZED: &str = "APPLICATION_DEAUTHORIZED";
pub const ENTITLEMENT_CREATE: &str = "ENTITLEMENT_CREATE";
pub const ENTITLEMENT_UPDATE: &str = "ENTITLEMENT_UPDATE";
pub const ENTITLEMENT_DELETE: &str = "ENTITLEMENT_DELETE";
pub const QUEST_USER_ENROLLMENT: &str = "QUEST_USER_ENROLLMENT";
pub const GAME_DIRECT_MESSAGE_CREATE: &str = "GAME_DIRECT_MESSAGE_CREATE";
pub const GAME_DIRECT_MESSAGE_UPDATE: &str = "GAME_DIRECT_MESSAGE_UPDATE";
pub const GAME_DIRECT_MESSAGE_DELETE: &str = "GAME_DIRECT_MESSAGE_DELETE";
pub const LOBBY_MESSAGE_CREATE: &str = "LOBBY_MESSAGE_CREATE";
pub const LOBBY_MESSAGE_UPDATE: &str = "LOBBY_MESSAGE_UPDATE";
pub const LOBBY_MESSAGE_DELETE: &str = "LOBBY_MESSAGE_DELETE";

/// Enum representing all Discord webhook event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DiscordEvent {
    // Application events
    #[serde(rename = "APPLICATION_AUTHORIZED")]
    ApplicationAuthorized(ApplicationEventData),
    #[serde(rename = "APPLICATION_DEAUTHORIZED")]
    ApplicationDeauthorized(ApplicationEventData),
    
    // Entitlement events (the inner event; see DiscordWebhookPayload for the full
    // POST body Discord actually sends, which wraps this under `event.data`).
    #[serde(rename = "ENTITLEMENT_CREATE")]
    EntitlementCreate(EntitlementEventData),
    #[serde(rename = "ENTITLEMENT_UPDATE")]
    EntitlementUpdate(EntitlementEventData),
    #[serde(rename = "ENTITLEMENT_DELETE")]
    EntitlementDelete(EntitlementEventData),
    
    // Quest events
    #[serde(rename = "QUEST_USER_ENROLLMENT")]
    QuestUserEnrollment(QuestEventData),
    
    // Game Direct Message events
    #[serde(rename = "GAME_DIRECT_MESSAGE_CREATE")]
    GameDirectMessageCreate(GameMessageEventData),
    #[serde(rename = "GAME_DIRECT_MESSAGE_UPDATE")]
    GameDirectMessageUpdate(GameMessageEventData),
    #[serde(rename = "GAME_DIRECT_MESSAGE_DELETE")]
    GameDirectMessageDelete(GameMessageEventData),
    
    // Lobby Message events
    #[serde(rename = "LOBBY_MESSAGE_CREATE")]
    LobbyMessageCreate(LobbyMessageEventData),
    #[serde(rename = "LOBBY_MESSAGE_UPDATE")]
    LobbyMessageUpdate(LobbyMessageEventData),
    #[serde(rename = "LOBBY_MESSAGE_DELETE")]
    LobbyMessageDelete(LobbyMessageEventData),
}

/// Data for application authorization events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplicationEventData {
    pub application_id: String,
    pub user_id: String,
    pub guild_id: Option<String>,
}

/// Data for entitlement events.
/// This models the `data` object inside ENTITLEMENT_* webhook events (and is
/// compatible with the shape returned by Discord's Entitlements REST API).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntitlementEventData {
    /// Discord snowflake ID of the entitlement (use this as the primary key).
    pub id: String,
    /// The user who received the entitlement.
    pub user_id: String,
    /// The SKU that was entitled.
    pub sku_id: String,
    /// The application that owns the SKU/entitlement.
    pub application_id: String,
    /// Entitlement type (see Discord docs: 1= purchase, 2=premium_sub, 3=gift, 4=test, etc.)
    /// Accepts "type" (from real Discord data JSON) on deserialze via alias;
    /// serializes as "entitlement_type" to avoid key conflict with outer event "type" discriminator.
    #[serde(rename = "entitlement_type", alias = "type")]
    pub entitlement_type: i32,
    /// Whether this is a test entitlement (created via API for testing).
    #[serde(default)]
    pub consumed: bool,
    /// Whether the entitlement has been deleted/revoked.
    #[serde(default)]
    pub deleted: bool,
    /// For subscriptions: when it starts.
    pub starts_at: Option<DateTime<Utc>>,
    /// For subscriptions: when it ends (null for lifetime).
    pub ends_at: Option<DateTime<Utc>>,
    // Additional fields from Discord (gift_code_flags, promotion_id, etc.) can be
    // added here as needed; kept minimal for now while covering premium use-case.
}

/// Data for quest enrollment events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestEventData {
    pub quest_id: String,
    pub user_id: String,
    /// Timestamp when the user enrolled in the quest
    pub enrolled_at: DateTime<Utc>,
}

/// Data for game direct message events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameMessageEventData {
    pub message_id: String,
    pub channel_id: String,
    pub author_id: String,
    pub content: String,
}

/// Data for lobby message events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LobbyMessageEventData {
    pub message_id: String,
    pub lobby_id: String,
    pub author_id: String,
    pub content: String,
}

impl DiscordEvent {
    /// Returns the event type as a string
    pub fn event_type(&self) -> &'static str {
        match self {
            DiscordEvent::ApplicationAuthorized(_) => APPLICATION_AUTHORIZED,
            DiscordEvent::ApplicationDeauthorized(_) => APPLICATION_DEAUTHORIZED,
            DiscordEvent::EntitlementCreate(_) => ENTITLEMENT_CREATE,
            DiscordEvent::EntitlementUpdate(_) => ENTITLEMENT_UPDATE,
            DiscordEvent::EntitlementDelete(_) => ENTITLEMENT_DELETE,
            DiscordEvent::QuestUserEnrollment(_) => QUEST_USER_ENROLLMENT,
            DiscordEvent::GameDirectMessageCreate(_) => GAME_DIRECT_MESSAGE_CREATE,
            DiscordEvent::GameDirectMessageUpdate(_) => GAME_DIRECT_MESSAGE_UPDATE,
            DiscordEvent::GameDirectMessageDelete(_) => GAME_DIRECT_MESSAGE_DELETE,
            DiscordEvent::LobbyMessageCreate(_) => LOBBY_MESSAGE_CREATE,
            DiscordEvent::LobbyMessageUpdate(_) => LOBBY_MESSAGE_UPDATE,
            DiscordEvent::LobbyMessageDelete(_) => LOBBY_MESSAGE_DELETE,
        }
    }
}

/// Outer payload that Discord actually POSTs to your Webhook Events URL.
///
/// See: https://discord.com/developers/docs/events/webhook-events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebhookPayload {
    pub version: u32,
    pub application_id: String,
    /// 0 = PING (must ack with 204), 1 = event (see `event` field).
    #[serde(rename = "type")]
    pub kind: u32,
    /// Present when kind == 1.
    pub event: Option<DiscordEventBody>,
}

/// The `event` wrapper inside a kind=1 payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEventBody {
    /// The specific event type, e.g. "ENTITLEMENT_CREATE".
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: String,
    /// The event-specific data. For entitlement events this will deserialize
    /// into an object matching EntitlementEventData (among others).
    pub data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Outer-envelope tests: the actual shape Discord POSTs to the
    //      Webhook Events URL. Inner-event tests below this block exercise
    //      DiscordEvent round-trip, but the wire shape is DiscordWebhookPayload.

    #[test]
    fn test_outer_envelope_ping_deserializes() {
        // Per Discord docs: kind=0 is a PING, sent once when you register the
        // Webhook Events URL. event must be absent / None.
        let body = r#"{
            "version": 1,
            "application_id": "1234567890123456789",
            "type": 0
        }"#;
        let payload: DiscordWebhookPayload = serde_json::from_str(body).unwrap();
        assert_eq!(payload.kind, 0);
        assert_eq!(payload.version, 1);
        assert!(payload.event.is_none(), "PING must not carry an event body");
    }

    #[test]
    fn test_outer_envelope_entitlement_create_real_shape() {
        // Realistic shape Discord actually POSTs for ENTITLEMENT_CREATE.
        // Key thing to lock down: the inner `data.type` field (an int, 1 = purchase)
        // must land in EntitlementEventData::entitlement_type via the
        // `#[serde(alias = "type")]` attribute — without it the outer `type`
        // discriminator would collide.
        let body = r#"{
            "version": 1,
            "application_id": "1234567890123456789",
            "type": 1,
            "event": {
                "type": "ENTITLEMENT_CREATE",
                "timestamp": "2026-06-01T20:00:00Z",
                "data": {
                    "id": "1100000000000000001",
                    "sku_id": "2200000000000000002",
                    "application_id": "1234567890123456789",
                    "user_id": "3300000000000000003",
                    "type": 1,
                    "deleted": false,
                    "starts_at": null,
                    "ends_at": null
                }
            }
        }"#;

        let payload: DiscordWebhookPayload = serde_json::from_str(body).unwrap();
        assert_eq!(payload.kind, 1);
        let event = payload.event.expect("kind=1 must carry an event body");
        assert_eq!(event.event_type, "ENTITLEMENT_CREATE");

        // Two-step deserialize: payload.event.data is Value (Discord's data
        // field varies by event type), so consumers re-deserialize into the
        // appropriate typed struct based on event_type.
        let ent: EntitlementEventData = serde_json::from_value(event.data).unwrap();
        assert_eq!(ent.id, "1100000000000000001");
        assert_eq!(
            ent.entitlement_type, 1,
            "inner data.type=1 must land in entitlement_type via serde alias"
        );
        assert_eq!(ent.user_id, "3300000000000000003");
        assert!(!ent.deleted);
        assert!(ent.starts_at.is_none());
        assert!(ent.ends_at.is_none());
    }

    #[test]
    fn test_outer_envelope_entitlement_delete_has_timestamps() {
        // ENTITLEMENT_DELETE typically carries a deleted=true flag and may
        // have ends_at set to indicate when access stopped.
        let body = r#"{
            "version": 1,
            "application_id": "1234567890123456789",
            "type": 1,
            "event": {
                "type": "ENTITLEMENT_DELETE",
                "timestamp": "2026-06-01T20:00:00Z",
                "data": {
                    "id": "1100000000000000001",
                    "sku_id": "2200000000000000002",
                    "application_id": "1234567890123456789",
                    "user_id": "3300000000000000003",
                    "type": 2,
                    "deleted": true,
                    "starts_at": "2026-01-01T00:00:00Z",
                    "ends_at": "2026-06-01T00:00:00Z"
                }
            }
        }"#;

        let payload: DiscordWebhookPayload = serde_json::from_str(body).unwrap();
        let event = payload.event.unwrap();
        assert_eq!(event.event_type, "ENTITLEMENT_DELETE");

        let ent: EntitlementEventData = serde_json::from_value(event.data).unwrap();
        assert!(ent.deleted);
        assert_eq!(ent.entitlement_type, 2);
        assert!(ent.starts_at.is_some());
        assert!(ent.ends_at.is_some());
    }

    #[test]
    fn test_application_authorized_serialization() {
        let event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
            application_id: "123".to_string(),
            user_id: "456".to_string(),
            guild_id: Some("789".to_string()),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("APPLICATION_AUTHORIZED"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_entitlement_create_serialization() {
        let event = DiscordEvent::EntitlementCreate(EntitlementEventData {
            id: "ent_123".to_string(),
            user_id: "user_456".to_string(),
            sku_id: "sku_789".to_string(),
            application_id: "app_012".to_string(),
            entitlement_type: 1,
            consumed: false,
            deleted: false,
            starts_at: None,
            ends_at: None,
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ENTITLEMENT_CREATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_quest_user_enrollment_serialization() {
        let enrolled_at = "2023-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let event = DiscordEvent::QuestUserEnrollment(QuestEventData {
            quest_id: "quest_123".to_string(),
            user_id: "user_456".to_string(),
            enrolled_at,
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("QUEST_USER_ENROLLMENT"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_game_direct_message_create_serialization() {
        let event = DiscordEvent::GameDirectMessageCreate(GameMessageEventData {
            message_id: "msg_123".to_string(),
            channel_id: "ch_456".to_string(),
            author_id: "user_789".to_string(),
            content: "Hello, world!".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("GAME_DIRECT_MESSAGE_CREATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_lobby_message_update_serialization() {
        let event = DiscordEvent::LobbyMessageUpdate(LobbyMessageEventData {
            message_id: "msg_123".to_string(),
            lobby_id: "lobby_456".to_string(),
            author_id: "user_789".to_string(),
            content: "Updated message".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("LOBBY_MESSAGE_UPDATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_event_type_method() {
        let event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
            application_id: "123".to_string(),
            user_id: "456".to_string(),
            guild_id: None,
        });
        
        assert_eq!(event.event_type(), "APPLICATION_AUTHORIZED");
    }

    #[test]
    fn test_application_deauthorized_serialization() {
        let event = DiscordEvent::ApplicationDeauthorized(ApplicationEventData {
            application_id: "app_123".to_string(),
            user_id: "user_456".to_string(),
            guild_id: Some("guild_789".to_string()),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("APPLICATION_DEAUTHORIZED"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_entitlement_update_serialization() {
        let event = DiscordEvent::EntitlementUpdate(EntitlementEventData {
            id: "ent_update_123".to_string(),
            user_id: "user_update_456".to_string(),
            sku_id: "sku_update_789".to_string(),
            application_id: "app_update_012".to_string(),
            entitlement_type: 2,
            consumed: false,
            deleted: false,
            starts_at: None,
            ends_at: Some("2025-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ENTITLEMENT_UPDATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_entitlement_delete_serialization() {
        let event = DiscordEvent::EntitlementDelete(EntitlementEventData {
            id: "ent_delete_123".to_string(),
            user_id: "user_delete_456".to_string(),
            sku_id: "sku_delete_789".to_string(),
            application_id: "app_delete_012".to_string(),
            entitlement_type: 1,
            consumed: false,
            deleted: true,
            starts_at: None,
            ends_at: None,
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ENTITLEMENT_DELETE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_game_direct_message_update_serialization() {
        let event = DiscordEvent::GameDirectMessageUpdate(GameMessageEventData {
            message_id: "msg_update_123".to_string(),
            channel_id: "ch_update_456".to_string(),
            author_id: "user_update_789".to_string(),
            content: "Updated game message".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("GAME_DIRECT_MESSAGE_UPDATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_game_direct_message_delete_serialization() {
        let event = DiscordEvent::GameDirectMessageDelete(GameMessageEventData {
            message_id: "msg_delete_123".to_string(),
            channel_id: "ch_delete_456".to_string(),
            author_id: "user_delete_789".to_string(),
            content: "Deleted game message".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("GAME_DIRECT_MESSAGE_DELETE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_lobby_message_create_serialization() {
        let event = DiscordEvent::LobbyMessageCreate(LobbyMessageEventData {
            message_id: "msg_create_123".to_string(),
            lobby_id: "lobby_create_456".to_string(),
            author_id: "user_create_789".to_string(),
            content: "Created lobby message".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("LOBBY_MESSAGE_CREATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_lobby_message_delete_serialization() {
        let event = DiscordEvent::LobbyMessageDelete(LobbyMessageEventData {
            message_id: "msg_delete_123".to_string(),
            lobby_id: "lobby_delete_456".to_string(),
            author_id: "user_delete_789".to_string(),
            content: "Deleted lobby message".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("LOBBY_MESSAGE_DELETE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }
}
