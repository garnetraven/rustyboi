use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let mentioned_user = match msg.mentions.first() {
        Some(user) => user,
        None => {
            msg.channel_id.say(&ctx.http, "Please mention a user.").await?;
            return Ok(())
        }
    };

    let user = match ctx.http.get_user(mentioned_user.id).await {
        Ok(user) => user,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Could not retrieve user info.").await?;
            return Ok(());
        }
    };

    let created_at = user.created_at().format("%Y-%m-%d %H:%M:%S UTC");

    let response = format!{"Username: {}\nUser ID: {}\nAccount created: {}", user.name, user.id, created_at};

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
