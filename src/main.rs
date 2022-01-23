pub mod bot;
use std::env;
use dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let username = env::var("USER").unwrap();
    let password = env::var("PASS").unwrap();
    let homeserver = env::var("SRV").unwrap();

    let bot = bot::Bot::new(username, password, homeserver).await;
    match bot {
        Ok(_) => println!("Logged in"),
        Err(e) => println!("{}", e),
    };

    Ok(())
}