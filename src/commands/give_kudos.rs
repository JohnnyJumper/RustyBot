use crate::prisma::{
    user,
    user_kudos::{self, from},
    PrismaClient,
};
use async_trait::async_trait;
use prisma_client_rust::{Direction, QueryError};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        },
        user::User,
    },
    utils::MessageBuilder,
};

use super::command::{CommandContext, UserCommand};
use chrono::{Datelike, TimeZone, Utc};

pub struct Command;

impl Command {
    fn unwrap_option(options: &Vec<CommandDataOption>) -> Option<User> {
        let mut result: Option<User> = None;
        let option = options
            .get(0)
            .expect("Expected user option")
            .resolved
            .as_ref()
            .expect("Expected user object");

        if let CommandDataOptionValue::User(user, _member) = option {
            let owned = user.to_owned();
            result = Some(owned);
        }

        result
    }

    async fn has_given_kudos_today(prisma: &PrismaClient, discord_user_id: String) -> bool {
        let latest_kudos = prisma
            .user_kudos()
            .find_many(vec![user_kudos::from_id::equals(user_id)])
            .order_by(user_kudos::timestamp::order(Direction::Desc))
            .take(1)
            .exec()
            .await;

        match latest_kudos {
            Ok(kudos) => {
                let kudos_timestamp = match kudos.get(0) {
                    Some(kudos) => Utc.timestamp_nanos(kudos.timestamp.timestamp_nanos()),
                    None => Utc::now(),
                };
                let today = Utc::now();

                if today.year() == kudos_timestamp.year()
                    && today.month() == kudos_timestamp.month()
                    && today.date_naive() == kudos_timestamp.date_naive()
                {
                    return true;
                }

                return false;
            }
            Err(why) => {
                println!("Silently ignoring this error: {:?}", why);
                false
            }
        }
    }
}

#[async_trait]
impl UserCommand for Command {
    async fn run(context: CommandContext<'async_trait>) -> String {
        let prisma = &context.client;
        let options = &context.options;

        let receiver = Command::unwrap_option(&options);
        let mut response = MessageBuilder::new();
        match receiver {
            Some(user) => {
                let receiver = user;

                let has_given_kudos_today =
                    Command::has_given_kudos_today(prisma, context.user.id.to_string());

                if let _False = Command::has_given_kudos_today(prisma, context.user.id.into()) {
                    let create_kudos = prisma
                        .user_kudos()
                        .create(
                            user::discord_user_id::equals(context.user.id.to_string()),
                            user::discord_user_id::equals(receiver.id.to_string()),
                            vec![],
                        )
                        .exec()
                        .await;
                    if let Ok(created_kudos) = create_kudos {
                        let _ = prisma
                            .user()
                            .update(
                                user::discord_user_id::equals(receiver.id.to_string()),
                                vec![user::reputation::increment(1.0)],
                            )
                            .exec()
                            .await;
                        response.push("Kudos has been given");
                    }
                }
            }
            None => {
                response.push("Failed to understand the receiver of kudos (unknown user)");
            }
        }

        response.build()
    }

    fn register<'a>(
        command: &'a mut CreateApplicationCommand,
        name: &'a str,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name(name)
            .description("give kudos to the user".to_string())
            .create_option(|option| {
                option
                    .kind(CommandOptionType::User)
                    .name("to_user")
                    .description("recipient of your kudos")
                    .required(true)
            })
    }
}
