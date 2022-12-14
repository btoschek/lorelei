mod handler;
mod modules;

use dotenv::dotenv;
use handler::BotHandler;
use serenity::{client::Client, framework::StandardFramework, prelude::GatewayIntents};
use songbird::SerenityInit;
use std::env;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize event logging for the bot itself
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("BOT_TOKEN").expect("Missing TOKEN");
    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    event!(Level::INFO, "Starting up.");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(BotHandler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| event!(Level::ERROR, "Client ended: {:?}", why));
    });

    // Wait for SIGINT to stop the bot
    #[allow(unused_must_use)]
    {
        tokio::signal::ctrl_c().await;
        event!(Level::INFO, "Received Ctrl-C, shutting down.");
    }
}
