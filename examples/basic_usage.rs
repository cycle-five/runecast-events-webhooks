use runecast_events_webhooks::{
    ApplicationEventData, DiscordEvent, EntitlementEventData, GameMessageEventData,
    LobbyMessageEventData, QuestEventData,
};

fn main() {
    // Example 1: Application Authorization Event
    let app_authorized = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
        application_id: "123456789".to_string(),
        user_id: "987654321".to_string(),
        guild_id: Some("555555555".to_string()),
    });
    println!("Application Authorized Event:");
    println!("{}", serde_json::to_string_pretty(&app_authorized).unwrap());
    println!();

    // Example 2: Entitlement Create Event
    let entitlement_create = DiscordEvent::EntitlementCreate(EntitlementEventData {
        entitlement_id: "ent_123456".to_string(),
        user_id: "user_789".to_string(),
        sku_id: "sku_abc".to_string(),
        application_id: "app_xyz".to_string(),
    });
    println!("Entitlement Create Event:");
    println!(
        "{}",
        serde_json::to_string_pretty(&entitlement_create).unwrap()
    );
    println!();

    // Example 3: Quest User Enrollment Event
    let quest_enrollment = DiscordEvent::QuestUserEnrollment(QuestEventData {
        quest_id: "quest_12345".to_string(),
        user_id: "user_67890".to_string(),
        enrolled_at: "2023-12-17T13:54:00Z".to_string(),
    });
    println!("Quest User Enrollment Event:");
    println!(
        "{}",
        serde_json::to_string_pretty(&quest_enrollment).unwrap()
    );
    println!();

    // Example 4: Game Direct Message Create Event
    let game_message = DiscordEvent::GameDirectMessageCreate(GameMessageEventData {
        message_id: "msg_11111".to_string(),
        channel_id: "ch_22222".to_string(),
        author_id: "user_33333".to_string(),
        content: "Hello from the game!".to_string(),
    });
    println!("Game Direct Message Create Event:");
    println!("{}", serde_json::to_string_pretty(&game_message).unwrap());
    println!();

    // Example 5: Lobby Message Update Event
    let lobby_message = DiscordEvent::LobbyMessageUpdate(LobbyMessageEventData {
        message_id: "msg_44444".to_string(),
        lobby_id: "lobby_55555".to_string(),
        author_id: "user_66666".to_string(),
        content: "Updated lobby message".to_string(),
    });
    println!("Lobby Message Update Event:");
    println!(
        "{}",
        serde_json::to_string_pretty(&lobby_message).unwrap()
    );
    println!();

    // Example 6: Parsing JSON into events
    let json_event = r#"{
        "type": "APPLICATION_DEAUTHORIZED",
        "application_id": "app_999",
        "user_id": "user_888",
        "guild_id": null
    }"#;

    match serde_json::from_str::<DiscordEvent>(json_event) {
        Ok(event) => {
            println!("Parsed event type: {}", event.event_type());
            println!("Event details: {:?}", event);
        }
        Err(e) => {
            println!("Failed to parse event: {}", e);
        }
    }
}
