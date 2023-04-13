use std::error::Error;

use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;

use serenity::utils::MessageBuilder;

use crate::prisma::user;

use super::command::CommandContext;
use super::command::ICommand;

pub struct Command;

#[async_trait]
impl ICommand for Command {
    async fn run(context: CommandContext<'async_trait>) -> String {
        let client = context.client;
        let user = context.user;
        let result = client
            .user()
            .create(
                user.name.clone(),
                user.id.to_string(),
                vec![user::reputation::set(100.0 as f64)],
            )
            .exec()
            .await;

        let response = match result {
            Ok(user) => MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&user.username)
                .push("has been ")
                .push_bold_safe("created ")
                .push("and has ")
                .push_bold_safe(user.reputation.to_string())
                .build(),
            Err(why) => MessageBuilder::new()
                .push("Error joining the ledger: ")
                .push_bold_safe(why)
                .build(),
        };

        response
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("join").description("join our ledger")
    }
}
