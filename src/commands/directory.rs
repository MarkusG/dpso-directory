use crate::planetside::*;

use serenity::{
    prelude::*,
    framework::standard::{
        macros::{group, command},
        CommandResult,
        Args
    },
    model::channel::Message
};

#[group]
#[commands(register)]
pub struct Directory;

// !register faction server name
#[command]
pub async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let faction_arg = args.single::<String>()?;
    let server_arg = args.single::<String>()?;
    let name = args.single::<String>()?;

    let faction = parse_faction(&faction_arg);
    if let None = faction {
        msg.channel_id.say(&ctx.http, "Invalid faction").await?;
        return Ok(())
    }
    let faction = faction.unwrap();

    let server = parse_server(&server_arg);
    if let None = server {
        msg.channel_id.say(&ctx.http, "Invalid server").await?;
        return Ok(())
    }
    let server = server.unwrap();

    msg.channel_id.say(&ctx.http, format!("{:?} {:?} {}", faction, server, name)).await?;
    Ok(())
}
