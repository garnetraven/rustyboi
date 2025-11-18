pub mod ping;

use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_message_command(ctx: &Context, msg: &Message, content: &str) {
    match content {
       "!ping" => ping::run(ctx, msg).await,
        _ => {}
    }
}
