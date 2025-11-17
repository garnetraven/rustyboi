use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    } 
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Log in with bot token from env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the env.");

    // Set gateway intents, which decides which events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, loggin in as the bot
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client.");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await  {
        println!("Client error {why:?}");
    }
}
