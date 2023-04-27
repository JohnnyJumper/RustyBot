use std::sync::Arc;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    http::Http,
    model::{prelude::interaction::application_command::CommandDataOption, user::User},
    utils::MessageBuilder,
};

use crate::prisma::PrismaClient;
use crate::utils::role::UserRole;

pub struct CommandContext<'a> {
    pub options: &'a Vec<CommandDataOption>,
    pub user: &'a User,
    pub user_role: UserRole,
    pub client: &'a PrismaClient,
    pub http: &'a Arc<Http>,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        options: &'a Vec<CommandDataOption>,
        user: &'a User,
        user_role: UserRole,
        client: &'a PrismaClient,
        http: &'a Arc<Http>,
    ) -> Self {
        Self {
            options,
            user,
            user_role,
            client,
            http,
        }
    }
}

#[async_trait]
pub trait UserCommand {
    fn register<'a>(
        command: &'a mut CreateApplicationCommand,
        name: &'a str,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name(name)
            .description(format!("A {} command", name))
    }
    async fn run(context: CommandContext<'async_trait>) -> String;
}

#[async_trait]
pub trait AdminCommand: UserCommand {
    async fn admin_logic(context: CommandContext<'async_trait>) -> String;
}

#[async_trait]
impl<T: AdminCommand + Sync> UserCommand for T {
    async fn run(context: CommandContext<'async_trait>) -> String {
        let user_role = &context.user_role;
        match user_role {
            UserRole::Admin => T::admin_logic(context).await,
            _ => MessageBuilder::new()
                .push("Only bot overseers allowed to use this command\n")
                .push_bold_line_safe("This action will be noted")
                .build(),
        }
    }
}
