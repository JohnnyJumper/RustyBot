mod commands;
mod prisma;

use std::env;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::user::User;
use serenity::prelude::*;

use crate::commands::command::{CommandContext, ICommand};

struct Handler {
    client: PrismaClient,
}

impl Handler {
    pub async fn new() -> Self {
        Self {
            client: PrismaClient::_builder().build().await.unwrap(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_context =
                CommandContext::new(&command.data.options, &command.user, &self.client);
            println!("Received command int eraction: {:#?}", command);
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::Command::run(command_context).await,
                "me" => commands::me::Command::run(command_context).await,
                "join" => commands::join::Command::run(command_context).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::Command::register(command));
            commands.create_application_command(|command| commands::me::Command::register(command));
            commands
                .create_application_command(|command| commands::join::Command::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() {
    let token =
        env::var("DISCORD_TOKEN").expect("Expected a token in the DISCORD_TOKEN environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler::new().await;

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating a client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
