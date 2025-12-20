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
    
    // Entitlement events
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

/// Data for entitlement events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntitlementEventData {
    pub entitlement_id: String,
    pub user_id: String,
    pub sku_id: String,
    pub application_id: String,
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

#[cfg(test)]
mod tests {
    use super::*;

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
            entitlement_id: "ent_123".to_string(),
            user_id: "user_456".to_string(),
            sku_id: "sku_789".to_string(),
            application_id: "app_012".to_string(),
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
            entitlement_id: "ent_update_123".to_string(),
            user_id: "user_update_456".to_string(),
            sku_id: "sku_update_789".to_string(),
            application_id: "app_update_012".to_string(),
        });
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ENTITLEMENT_UPDATE"));
        
        let deserialized: DiscordEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn test_entitlement_delete_serialization() {
        let event = DiscordEvent::EntitlementDelete(EntitlementEventData {
            entitlement_id: "ent_delete_123".to_string(),
            user_id: "user_delete_456".to_string(),
            sku_id: "sku_delete_789".to_string(),
            application_id: "app_delete_012".to_string(),
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
