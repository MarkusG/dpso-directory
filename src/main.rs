mod commands;
mod planetside;

use std::{fs, process};

use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*
};

use tracing::{info, error};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = fs::read_to_string("token.txt");
    if let Err(e) = token {
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                error!("token.txt not found");
            },
            _ => {
                error!("Could not read token.txt: {:?}", e);
            }

        }
        process::exit(1);
    }
    let token = token.unwrap();

    let framework = commands::build_framework();

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
