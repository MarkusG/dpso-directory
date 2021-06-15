use serenity::{
    prelude::*,
    framework::standard::{
        macros::{group, command},
        CommandResult
    },
    model::channel::Message
};

#[group]
#[commands(ping)]
pub struct Directory;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}
