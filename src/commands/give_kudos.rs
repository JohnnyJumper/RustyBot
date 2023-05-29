use super::command::{CommandContext, UserCommand};
use crate::{
    prisma::{kudos, user, PrismaClient},
    responses,
};
use async_trait::async_trait;
use chrono::{Datelike, TimeZone, Utc};
use prisma_client_rust::Direction;
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

pub struct Command;

impl Command {
    async fn give_kudos(
        prisma: &PrismaClient,
        sender: &User,
        receiver: &User,
        message: Option<String>,
    ) -> String {
        let response: String;
        let user_message = message.unwrap_or("No message was provided.".to_string());
        let db_response = prisma
            ._batch((
                prisma.kudos().create(
                    user::discord_user_id::equals(sender.id.to_string()),
                    user::discord_user_id::equals(receiver.id.to_string()),
                    user_message.clone(),
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
                    &user_message,
                );
            }
            Err(why) => {
                response = responses::errors::general_unknown_error_message(why.to_string());
            }
        }
        response
    }

    fn unwrap_options(options: &Vec<CommandDataOption>) -> Option<(User, Option<String>)> {
        let mut result: Option<(User, Option<String>)> = None;
        let option_user = options
            .get(0)
            .expect("Expected user option")
            .resolved
            .as_ref()
            .expect("Expected user object");

        let option_message = options.get(1);

        if let CommandDataOptionValue::User(user, _member) = option_user {
            let owned = user.to_owned();

            if let Some(data_option) = option_message {
                let option_message = data_option
                    .resolved
                    .as_ref()
                    .expect("Expected message object");

                if let CommandDataOptionValue::String(message) = option_message {
                    result = Some((owned, Some(message.to_owned())));
                } else {
                    result = Some((owned, None));
                };
            };
        };
        println!("result after the big block: {:?}", result);
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
        let options = Command::unwrap_options(&context.options);
        let response: String;

        match options {
            Some((receiver, optional_message)) => {
                let has_given_kudos_today =
                    Command::has_given_kudos_today(prisma, sender.id.to_string()).await;
                response = if sender.id.eq(&receiver.id) {
                    responses::errors::cannot_increase_your_own_reputation_message()
                } else if has_given_kudos_today {
                    responses::errors::only_one_kudos_error_message()
                } else {
                    Command::give_kudos(prisma, sender, &receiver, optional_message).await
                };
            }
            None => {
                response = responses::errors::general_unknown_error_message(
                    "invalid arguments, buddy".to_owned(),
                );
            }
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
            .create_option(|option| {
                option
                    .kind(CommandOptionType::String)
                    .name("message")
                    .description("Usually why you are giving someone a kudos")
                    .required(true)
            })
    }
}
