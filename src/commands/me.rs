use crate::prisma::user;
use async_trait::async_trait;
use serenity::utils::MessageBuilder;

use super::command::{CommandContext, UserCommand};

pub struct Command;

#[async_trait]
impl UserCommand for Command {
    async fn run(context: CommandContext<'async_trait>) -> String {
        let client = context.client;
        let user = context.user;
        println!("userId {:#?}", user.id.to_string());

        let db_user = client
            .user()
            .find_first(vec![user::discord_user_id::equals(user.id.to_string())])
            .exec()
            .await;

        let response = match db_user {
            Ok(user_option) => match user_option {
                Some(user) => {
                    let reputation = user.reputation.to_string();
                    MessageBuilder::new()
                        .push("User ")
                        .push_bold_safe(&user.username)
                        .push(" has the following reputation of ")
                        .push_bold_safe(&reputation)
                        .build()
                }
                None => MessageBuilder::new()
                    .push("User ")
                    .push_bold_line_safe(&user.name)
                    .push(" is not registered, talk with admins")
                    .build(),
            },
            Err(why) => MessageBuilder::new()
                .push("Error looking into db for user ")
                .push_bold_line_safe(&user.name)
                .push(" : ")
                .push(why)
                .build(),
        };
        response
    }
}
