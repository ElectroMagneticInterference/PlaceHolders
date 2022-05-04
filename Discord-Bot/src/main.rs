use std::env;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{macros::hook, StandardFramework},
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use std::sync::Arc;
use tokio;

use tracing::{debug, info, instrument};

pub mod util;
mod commands;
use commands::*;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

#[hook]
#[instrument]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true
}

#[tokio::main]
async fn main() {
    kankyo::load(true).expect("Failed to load .env file");

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("~")
                .allow_dm(true)
                .case_insensitivity(false)
                .with_whitespace(true)
        })
        .before(before)
        .group(&GENERAL_GROUP)
        .group(&OWNERS_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    {
        let shard_manager = client.shard_manager.clone();

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
        });
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
