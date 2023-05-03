use super::command::{AdminCommand, CommandContext};
use crate::{responses, utils::role::GUILD_ID};
use async_trait::async_trait;
use serenity::utils::MessageBuilder;

pub struct Command;

#[async_trait]
impl AdminCommand for Command {
    async fn admin_logic(context: CommandContext<'async_trait>) -> String {
        let client = context.client;
        let mut response = MessageBuilder::new();
        // first get all guild members
        match GUILD_ID.members(context.http, None, None).await {
            Ok(members) => {
                for member in members {
                    match client
                        .user()
                        .create(
                            member.display_name().to_string(),
                            member.user.id.to_string(),
                            vec![],
                        )
                        .exec()
                        .await
                    {
                        Ok(user) => {
                            response
                                .push(format!("{:<15} | ", member.display_name().to_string()))
                                .push(format!("{:<4} | ", member.user.id.to_string()))
                                .push(format!("{}\n", user.reputation));
                        }
                        Err(why) => {
                            response
                                .push(format!("{:<15} | ", member.display_name().to_string()))
                                .push(format!("{:<4} | ", member.user.id.to_string()))
                                .push_bold_safe("was not created because ".to_string())
                                .push(format!("{} \n", why));
                        }
                    }
                }
                response.build()
            }
            Err(why) => responses::errors::error_retriving_guild_members_message(why.to_string()),
        }
    }
}
