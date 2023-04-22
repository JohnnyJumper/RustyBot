use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{prelude::interaction::application_command::CommandDataOption, user::User},
};

use crate::prisma::PrismaClient;
use crate::utils::role::UserRole;

pub struct CommandContext<'a> {
    pub options: &'a Vec<CommandDataOption>,
    pub user: &'a User,
    pub user_role: UserRole,
    pub client: &'a PrismaClient,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        options: &'a Vec<CommandDataOption>,
        user: &'a User,
        user_role: UserRole,
        client: &'a PrismaClient,
    ) -> Self {
        Self {
            options,
            user,
            user_role,
            client,
        }
    }
}

#[async_trait]
pub trait ICommand {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    async fn run(context: CommandContext<'async_trait>) -> String;
}

#[async_trait]
pub trait IAdminCommand: ICommand {
    async fn admin_logic(&self, context: CommandContext<'async_trait>) -> String;

    async fn run(&self, context: CommandContext<'async_trait>) -> String {
        let user = context.user;

        self.admin_logic(context).await
    }
}
