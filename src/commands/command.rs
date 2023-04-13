use std::error::Error;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::{prelude::interaction::application_command::CommandDataOption, user::User},
};

use crate::prisma::PrismaClient;

pub struct CommandContext<'a> {
    pub options: &'a Vec<CommandDataOption>,
    pub user: &'a User,
    pub client: &'a PrismaClient,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        options: &'a Vec<CommandDataOption>,
        user: &'a User,
        client: &'a PrismaClient,
    ) -> Self {
        Self {
            options,
            user,
            client,
        }
    }
}

#[async_trait]
pub trait ICommand {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    async fn run(context: CommandContext<'async_trait>) -> String;
}
