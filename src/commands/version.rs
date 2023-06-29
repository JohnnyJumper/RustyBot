use super::command::{CommandContext, UserCommand};
use crate::{prisma::user, responses};
use async_trait::async_trait;

pub struct Command;

#[async_trait]
impl UserCommand for Command {
    async fn run(context: CommandContext<'async_trait>) -> String {
        const version: &str = env!("CARGO_PKG_VERSION");
        format!("My current version is {}", version)
    }
}
