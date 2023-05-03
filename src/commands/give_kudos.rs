use crate::{
    prisma::{self, kudos, user, PrismaClient},
    responses,
};

use prisma::kudos::Data as kudosData;
use prisma::user::Data as userData;

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
};

use super::command::{CommandContext, UserCommand};
use chrono::{Datelike, TimeZone, Utc};

pub struct Command;

impl Command {
    async fn give_kudos(prisma: &PrismaClient, sender: &User, receiver: &User) -> String {
        let response: String;
        let db_response = prisma
            ._batch((
                prisma.kudos().create(
                    user::discord_user_id::equals(sender.id.to_string()),
                    user::discord_user_id::equals(receiver.id.to_string()),
                    vec![],
                ),
                prisma
                    .user()
                    .update(
                        user::discord_user_id::equals(receiver.id.to_string()),
                        vec![user::reputation::increment(1.0)],
                    )
                    .select(user::select!({ reputation })),
            ))
            .await;

        match db_response {
            Ok(data) => {
                response = responses::general::succesfully_given_kudos_message(
                    &receiver.name,
                    data.1.reputation,
                );
            }
            Err(why) => {
                response = responses::errors::general_unknown_error_message(why.to_string());
            }
        }
        response
    }

    fn unwrap_options(options: &Vec<CommandDataOption>) -> Option<User> {
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
            .kudos()
            .find_many(vec![kudos::from_discord_id::equals(discord_user_id)])
            .order_by(kudos::timestamp::order(Direction::Desc))
            .take(1)
            .exec()
            .await;
        println!("{:?}", latest_kudos);
        match latest_kudos {
            Ok(kudos) => {
                if kudos.len() == 0 {
                    return false;
                }
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
        let prisma = context.client;
        let sender = context.user;
        let receiver = Command::unwrap_options(&context.options);
        let response;

        match receiver {
            Some(user) => {
                let receiver = &user;
                let has_given_kudos_today =
                    Command::has_given_kudos_today(prisma, sender.id.to_string()).await;

                response = if sender.id.eq(&receiver.id) {
                    responses::errors::cannot_increase_your_own_reputation_message()
                } else if has_given_kudos_today {
                    responses::errors::only_one_kudos_error_message()
                } else {
                    Command::give_kudos(prisma, sender, receiver).await
                };
            }
            None => response = responses::errors::unknown_user_error_message(),
        }

        response
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
