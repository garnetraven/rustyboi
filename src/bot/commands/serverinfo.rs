use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let guild_id = match msg.guild_id{
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "This command can only be used in a server.").await?;
            return Ok(())
        }
    };

    let guild = guild_id.to_partial_guild(&ctx.http).await?;
    
    let guild_name = guild.name.clone();
    let member_count = guild.approximate_member_count;

    let response = format!{"Server name: {}\nServer ID: {}\nMembers: {:?}", guild_name, guild.id, member_count};

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
