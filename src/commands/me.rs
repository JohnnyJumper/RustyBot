use super::command::{CommandContext, UserCommand};
use crate::{prisma::user, responses};
use async_trait::async_trait;

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
                Some(user) => responses::general::me_command_response(
                    &user.username,
                    &user.reputation.to_string(),
                ),
                None => responses::errors::not_registered_message(&user.name),
            },
            Err(why) => responses::errors::user_not_found_message(&user.name, why.to_string()),
        };
        response
    }
}
