pub mod bot;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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