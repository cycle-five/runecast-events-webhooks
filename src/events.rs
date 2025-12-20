use serde::{Deserialize, Serialize};

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
    /// ISO 8601 timestamp string (e.g., "2023-01-01T00:00:00Z")
    pub enrolled_at: String,
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
            DiscordEvent::ApplicationAuthorized(_) => "APPLICATION_AUTHORIZED",
            DiscordEvent::ApplicationDeauthorized(_) => "APPLICATION_DEAUTHORIZED",
            DiscordEvent::EntitlementCreate(_) => "ENTITLEMENT_CREATE",
            DiscordEvent::EntitlementUpdate(_) => "ENTITLEMENT_UPDATE",
            DiscordEvent::EntitlementDelete(_) => "ENTITLEMENT_DELETE",
            DiscordEvent::QuestUserEnrollment(_) => "QUEST_USER_ENROLLMENT",
            DiscordEvent::GameDirectMessageCreate(_) => "GAME_DIRECT_MESSAGE_CREATE",
            DiscordEvent::GameDirectMessageUpdate(_) => "GAME_DIRECT_MESSAGE_UPDATE",
            DiscordEvent::GameDirectMessageDelete(_) => "GAME_DIRECT_MESSAGE_DELETE",
            DiscordEvent::LobbyMessageCreate(_) => "LOBBY_MESSAGE_CREATE",
            DiscordEvent::LobbyMessageUpdate(_) => "LOBBY_MESSAGE_UPDATE",
            DiscordEvent::LobbyMessageDelete(_) => "LOBBY_MESSAGE_DELETE",
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
        let event = DiscordEvent::QuestUserEnrollment(QuestEventData {
            quest_id: "quest_123".to_string(),
            user_id: "user_456".to_string(),
            enrolled_at: "2023-01-01T00:00:00Z".to_string(),
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
