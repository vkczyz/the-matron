use std::env;
use url::{Url};

use matrix_sdk::{
    Client, SyncSettings, Result,
    ruma::{
        events::{
            SyncMessageEvent, 
            room::message::{MessageEventContent},
        }
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("USER").unwrap();
    let password = env::var("PASS").unwrap();
    let homeserver = env::var("SRV").unwrap();

    let client = login(username, password, homeserver).await.unwrap();
    client.sync_once(SyncSettings::default()).await?;
    setup(&client).await;

    client.register_event_handler(|ev : SyncMessageEvent<MessageEventContent>| async move {
        on_room_message(ev).await;
    }).await;

    println!("Syncing");
    client.sync(SyncSettings::default()).await;

    Ok(())
}

async fn login(username: String, password: String, homeserver: String) -> Result<matrix_sdk::Client> {
    let homeserver_url = Url::parse(&homeserver)
        .expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url)
        .expect("Couldn't create the client");

    client.login(&username, &password, None, Some("Bot session")).await
        .expect("Couldn't log in");

    println!("Logged in as {} at {}", username, homeserver);
    Ok(client)
}

async fn setup(client: &matrix_sdk::Client) {
    for room in client.joined_rooms() {
        let message = MessageEventContent::text_plain("TEST");
        let _response = room.send(message, None).await;
    }
}

async fn on_room_message(event: SyncMessageEvent<MessageEventContent>) {
    print!("{:?}", event);
}