use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;

use super::command::{CommandContext, ICommand};

pub struct Command;

#[async_trait]
impl ICommand for Command {
    async fn run(_context: CommandContext<'async_trait>) -> String {
        "Hey, I'm alive!".to_string()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}
