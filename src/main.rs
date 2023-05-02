mod commands;
mod prisma;
mod responses;
mod utils;

use std::env;

use prisma::PrismaClient;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::id::RoleId;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::*;

use crate::{
    commands::command::{CommandContext, UserCommand},
    utils::role::{identify_role, UserRole, GUILD_ID},
};

struct Handler {
    client: PrismaClient,
}

impl Handler {
    pub async fn new() -> Handler {
        Self {
            client: PrismaClient::_builder().build().await.unwrap(),
        }
    }
}

macro_rules! run_command {
    ($command_name: expr, $command_context: expr, [$($command:ident),*]) => {
        match $command_name.as_str() {
            $(
                stringify!($command) => commands::$command::Command::run($command_context).await,
            )+
            &_ => "Unknown command".to_string()
        }
    };
}

macro_rules! register_slash_commands {
    ($commands:expr, [$($cmd:ident),+]) => {
        $(
            $commands.create_application_command(|command| crate::commands::$cmd::Command::register(command, stringify!($cmd)));
        )+
    };
}

async fn register_commands(ctx: &Context, guild_id: GuildId) {
    let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        register_slash_commands!(commands, [me, add_members, give_kudos]);
        commands
    })
    .await;
    println!(
        "I now have the following guild slash commands: {:#?}",
        commands
    );
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let role = match identify_role(&command.user, &ctx.http).await {
                Ok(role) => role,
                Err(why) => UserRole::Unknown(why.to_string()),
            };

            if let UserRole::Unknown(why) = role {
                let _ = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .content(crate::responses::general::unknown_role_message(why))
                            })
                    })
                    .await;
                return;
            }

            let command_context = CommandContext::new(
                &command.data.options,
                &command.user,
                role,
                &self.client,
                &ctx.http,
            );
            println!(
                "Received command interaction: {:#?} with options: {:#?}",
                command.data.name, command.data.options
            );
            let content = run_command!(
                command.data.name,
                command_context,
                [me, add_members, give_kudos]
            );
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

        register_commands(&ctx, *GUILD_ID).await;
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
