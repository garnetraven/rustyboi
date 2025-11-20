use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    msg.channel_id.say(&ctx.http, "Pong!").await?; 
    Ok(())
}
