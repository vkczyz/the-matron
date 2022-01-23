use std::error::Error;
use url::{Url};
use matrix_sdk::{
    Client, SyncSettings, ClientConfig,
    ruma::{
        events::{
            SyncMessageEvent, 
            room::message::{MessageEventContent},
        }
    },
};

pub struct Bot {
    username: String,
    password: String,
    homeserver: String,
}

impl Bot {
    pub async fn new(username: String, password: String, homeserver: String) -> Result<Self, Box<dyn Error>> {
        let config = ClientConfig::new().store_path(".");

        let bot = Bot {
            username,
            password,
            homeserver,
        };

        let client = bot.login(config).await.unwrap();

        client.sync_once(SyncSettings::default()).await?;
        Bot::setup(&client).await;

        client.register_event_handler(|ev : SyncMessageEvent<MessageEventContent>| async move {
            Bot::on_room_message(ev).await;
        }).await;

        println!("Syncing");
        client.sync(SyncSettings::default()).await;

        Ok(bot)
    }

    async fn login(&self, config: ClientConfig) -> Result<matrix_sdk::Client, Box<dyn Error>> {
        let homeserver_url = Url::parse(&self.homeserver)
            .expect("Couldn't parse the homeserver URL");
        let client = Client::new_with_config(homeserver_url, config)
            .expect("Couldn't create the client");

        client.login(&self.username, &self.password, None, Some("Bot session")).await
            .expect("Couldn't log in");

        println!("Logged in as {} at {}", &self.username, &self.homeserver);
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
}