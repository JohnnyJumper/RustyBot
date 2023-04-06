use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#")) // set the bot's prefix to "#"
        .group(&GENERAL_GROUP);

    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in the DISCORD_TOKEN environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating a client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{:?}", msg);
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
