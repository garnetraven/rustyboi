use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod commands;

pub async fn start_bot() -> anyhow::Result<()> {
    dotenv().ok();

    // Log in with bot token from env
    let token = env::var("DISCORD_TOKEN")?;

    // Set gateway intents, which decides which events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, loggin in as the bot
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    client.start().await?;
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        commands::handle_message_command(&ctx, &msg, &msg.content).await;
    } 
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }
}

