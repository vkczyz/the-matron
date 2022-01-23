use std::env;
use url::{Url};

use matrix_sdk::{
    Client, SyncSettings, Result,
    ruma::{events::{SyncMessageEvent, room::message::MessageEventContent}},
};

#[tokio::main]
async fn main() -> Result<()> {

    let username = env::var("USER").unwrap();
    let password = env::var("PASS").unwrap();
    let homeserver = env::var("SRV").unwrap();

    let homeserver_url = Url::parse(&homeserver).expect("Couldn't parse the homeserver URL");
    let client = Client::new(homeserver_url).unwrap();

    client.login(&username, &password, None, Some("Bot session")).await?;

    println!("Logged in as {} at {}", username, homeserver);

    client
        .register_event_handler(
            |ev: SyncMessageEvent<MessageEventContent>| async move {
                println!("Received a message {:?}", ev);
            },
        )
        .await;

    println!("Syncing");

    client.sync(SyncSettings::default()).await;

    Ok(())
}
