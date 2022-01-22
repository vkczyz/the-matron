use std::convert::TryFrom;
use matrix_sdk::{
    Client, SyncSettings, Result,
    ruma::{UserId, events::{SyncMessageEvent, room::message::MessageEventContent}},
};

#[tokio::main]
async fn main() -> Result<()> {
    let user = UserId::try_from("@alice:example.org")?;
    let client = Client::new_from_user_id(user.clone()).await?;

    // First we need to log in.
    client.login(user.localpart(), "password", None, None).await?;

    client
        .register_event_handler(
            |ev: SyncMessageEvent<MessageEventContent>| async move {
                println!("Received a message {:?}", ev);
            },
        )
        .await;

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    client.sync(SyncSettings::default()).await;

    Ok(())
}
