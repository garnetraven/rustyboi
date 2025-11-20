pub mod ping;
pub mod userinfo;
pub mod serverinfo;

use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_message_command(ctx: &Context, msg: &Message, content: &str) {
    match content {
       "!ping" => {
            if let Err(e) = ping::run(ctx, msg).await {
                eprintln!("Ping command failed: {:?}", e);
            }
        }
        c if c.starts_with("!userinfo") => {
            if let Err(e) = userinfo::run(ctx, msg).await {
                eprintln!("Userinfo command failed: {:?}", e);
            }
        }
        "!serverinfo" => {
            if let Err(e) = serverinfo::run(ctx, msg).await {
                eprintln!("Serverinfo command failed: {:?}", e);
            }
        }
        _ => {}
    }
}
