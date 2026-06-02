use discord_webhook_events::{
    ApplicationEventData, DiscordEvent, DiscordWebhookPayload, EntitlementEventData,
    GameMessageEventData, LobbyMessageEventData, PartialUser, QuestEventData,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Receiving webhooks (the primary use-case) ---

    // Example 1: PING — Discord sends this once when you register your webhook URL.
    // Respond with 204 No Content to confirm the endpoint.
    let ping_body = r#"{"version":1,"application_id":"1234560123453231555","type":0}"#;
    let ping: DiscordWebhookPayload = serde_json::from_str(ping_body)?;
    assert_eq!(ping.kind, 0, "PING has kind=0");
    println!("PING received — ack with 204");

    // Example 2: ENTITLEMENT_CREATE — the most common event for a monetized app.
    let entitlement_body = r#"{
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
    }"#;
    let payload: DiscordWebhookPayload = serde_json::from_str(entitlement_body)?;
    match payload.event.as_ref().map(|b| &b.event) {
        Some(DiscordEvent::EntitlementCreate(ent)) => {
            println!("New entitlement: id={} sku={} user={}", ent.id, ent.sku_id, ent.user_id);
        }
        None => println!("PING"),
        _ => println!("Other event"),
    }

    // Example 3: APPLICATION_AUTHORIZED — user installs your app.
    // Note: application_id comes from the outer envelope; event.data has user + scopes.
    let auth_body = r#"{
        "version": 1,
        "application_id": "1234560123453231555",
        "type": 1,
        "event": {
            "type": "APPLICATION_AUTHORIZED",
            "timestamp": "2026-06-01T20:00:00Z",
            "data": {
                "integration_type": 1,
                "scopes": ["applications.commands"],
                "user": {
                    "id": "3300000000000000003",
                    "username": "exampleuser",
                    "global_name": "Example User",
                    "avatar": null
                }
            }
        }
    }"#;
    let payload: DiscordWebhookPayload = serde_json::from_str(auth_body)?;
    let app_id = &payload.application_id;
    match payload.event.as_ref().map(|b| &b.event) {
        Some(DiscordEvent::ApplicationAuthorized(app)) => {
            println!(
                "App {} authorized by user {} (scopes: {})",
                app_id,
                app.user.id,
                app.scopes.join(", ")
            );
        }
        _ => {}
    }

    // --- Constructing events (e.g. for tests or mocking) ---

    // Example 4: Build an entitlement event and round-trip it.
    let event = DiscordEvent::EntitlementCreate(EntitlementEventData {
        id: "ent_123456".to_string(),
        user_id: "user_789".to_string(),
        sku_id: "sku_abc".to_string(),
        application_id: "app_xyz".to_string(),
        entitlement_type: 1,
        consumed: false,
        deleted: false,
        starts_at: None,
        ends_at: None,
    });
    println!("\nEntitlement JSON:\n{}", serde_json::to_string_pretty(&event)?);

    // Example 5: Other event types.
    let _app_event = DiscordEvent::ApplicationAuthorized(ApplicationEventData {
        integration_type: 1,
        scopes: vec!["applications.commands".to_string()],
        user: PartialUser {
            id: "3300000000000000003".to_string(),
            username: "exampleuser".to_string(),
            global_name: Some("Example User".to_string()),
            avatar: None,
        },
        guild: None,
    });

    let _quest = DiscordEvent::QuestUserEnrollment(QuestEventData {
        quest_id: "quest_12345".to_string(),
        user_id: "user_67890".to_string(),
        enrolled_at: "2026-06-01T00:00:00Z".parse()?,
    });

    let _game_msg = DiscordEvent::GameDirectMessageCreate(GameMessageEventData {
        message_id: "msg_11111".to_string(),
        channel_id: "ch_22222".to_string(),
        author_id: "user_33333".to_string(),
        content: "Hello from the game!".to_string(),
    });

    let _lobby_msg = DiscordEvent::LobbyMessageUpdate(LobbyMessageEventData {
        message_id: "msg_44444".to_string(),
        lobby_id: "lobby_55555".to_string(),
        author_id: "user_66666".to_string(),
        content: "Updated lobby message".to_string(),
    });

    println!("\nEvent type: {}", event.event_type());
    Ok(())
}
