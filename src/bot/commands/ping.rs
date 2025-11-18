use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) {
    if let Err(error) = msg.channel_id.say(&ctx.http, "Pong!").await {
        eprintln!("Error: {:?}", error);
    }
}
