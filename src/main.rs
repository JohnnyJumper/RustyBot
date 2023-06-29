mod commands;
mod prisma;
mod responses;
mod utils;

use crate::{
    commands::command::{CommandContext, UserCommand},
    utils::role::{identify_role, UserRole, GUILD_ID},
};
use prisma::PrismaClient;
use serenity::{
    async_trait,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, Interaction,
            InteractionResponseType,
        },
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};
use std::env;

pub const DISCORD_RESPONSE_LIMIT: usize = 1996;

struct Handler {
    client: PrismaClient,
}

impl Handler {
    pub async fn new() -> Handler {
        Self {
            client: PrismaClient::_builder().build().await.unwrap(),
        }
    }

    pub async fn split_and_send_content(
        content: String,
        command: ApplicationCommandInteraction,
        ctx: &Context,
    ) {
        println!("splitting and sending?");
        println!("{}", &content);

        let chars = content.chars();
        let chars_amount = chars.count();
        let more_than_one_message = chars_amount > DISCORD_RESPONSE_LIMIT;
        let mut messages: Vec<String> = Vec::new();

        if more_than_one_message {
            let mut start = 0;
            while start < chars_amount {
                let end = std::cmp::min(start + DISCORD_RESPONSE_LIMIT, chars_amount);
                messages.push(content[start..end].to_string());
                start += DISCORD_RESPONSE_LIMIT;
            }
            let first_message = messages.remove(0);

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(first_message)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }

            for next_message in &messages {
                if let Err(why) = command
                    .create_followup_message(&ctx.http, |response| response.content(next_message))
                    .await
                {
                    println!("Error sending follow-up message {}", why);
                }
            }
        } else {
            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| response.content(content))
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
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
        register_slash_commands!(
            commands,
            [me, add_members, give_kudos, kudos_received, kudos_sent]
        );
        commands
    })
    .await;
    println!(
        "I now have {:#?} guild slash commands",
        commands.unwrap().len()
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
                "Received command interaction: {:#?} with {:#?} options",
                command.data.name,
                command.data.options.len()
            );

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(responses::general::work_in_progress_message())
                        })
                })
                .await
            {
                println!("Cannot defer slash command: {}", why);
                return;
            }

            let content = run_command!(
                command.data.name,
                command_context,
                [me, add_members, give_kudos, kudos_received, kudos_sent]
            );
            Handler::split_and_send_content(content, command, &ctx).await;
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
