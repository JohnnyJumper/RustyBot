use super::command::{CommandContext, UserCommand};
use crate::responses;
use async_trait::async_trait;

pub struct Command;

#[async_trait]
impl UserCommand for Command {
    async fn run(_context: CommandContext<'async_trait>) -> String {
        responses::general::contribute_message()
    }
}
