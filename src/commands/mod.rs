pub mod directory;

use serenity::{
    model::{
        channel::Message,
        prelude::Channel::{Guild, Private},
    },
    prelude::*,

    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::hook
    }
};

use tracing::{info, warn};

use directory::*;

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    let channel = match msg.channel_id.to_channel(&ctx.http).await {
        Ok(c) => {
            match c {
                Guild(g) => format!("#{}", g.name).to_string(),
                Private(_) => String::from("a private message"),
                _ => String::from("???")
            }
        }
        Err(_) => String::from("???")
    };

    info!("Executing {} for {}#{} in {}",
          command_name,
          msg.author.name,
          msg.author.discriminator,
          channel);
    true
}

#[hook]
async fn after(_: &Context, _: &Message, command_name: &str, command_result: CommandResult) {
    if let Err(e) = command_result {
        warn!("Failed executing {}: {:?}", command_name, e);
    }
}

pub fn build_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .prefix("!")
                   .delimiter(" "))
        .before(before)
        .after(after)
        .group(&DIRECTORY_GROUP)
}
