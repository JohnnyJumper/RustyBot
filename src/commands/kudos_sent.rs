pub struct Command;
use std::{fmt, vec};

use async_trait::async_trait;
use prisma_client_rust::Direction;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
    utils::MessageBuilder,
};

use crate::prisma::kudos;

use super::command::{CommandContext, UserCommand};

#[derive(PartialEq)]
enum OptionChoice {
    Last,
    First,
    Unknown,
}

impl fmt::Display for OptionChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let OptionChoice::First = self {
            write!(f, "first")
        } else {
            write!(f, "last")
        }
    }
}

impl Command {
    fn parse_choice_string(choice: &String) -> OptionChoice {
        if choice.eq("last") {
            return OptionChoice::Last;
        } else if choice.eq("first") {
            return OptionChoice::First;
        }
        return OptionChoice::Unknown;
    }

    fn unwrap_options(options: &Vec<CommandDataOption>) -> Option<(OptionChoice, i64)> {
        let amount = options
            .get(1)
            .expect("Amount int to be passed")
            .resolved
            .as_ref()
            .expect("Amount to be passed");

        if let CommandDataOptionValue::Integer(amount) = amount {
            let order_choice = options
                .get(0)
                .expect("Option object to be passed")
                .resolved
                .as_ref()
                .expect("Order choice enum option");
            if let CommandDataOptionValue::String(order) = order_choice {
                let choice = Self::parse_choice_string(order);
                if let OptionChoice::Unknown = choice {
                    return None;
                } else {
                    return Some((choice, *amount));
                }
            }
        }
        None
    }
}

#[async_trait]
impl UserCommand for Command {
    async fn run(context: CommandContext<'async_trait>) -> String {
        let prisma = context.client;
        let mut response = MessageBuilder::new();
        let options = Command::unwrap_options(&context.options);

        match options {
            Some((choice, amount)) => {
                let direction = if let OptionChoice::First = choice {
                    Direction::Asc
                } else {
                    Direction::Desc
                };

                let db_result = prisma
                    .kudos()
                    .find_many(vec![kudos::from_discord_id::equals(
                        context.user.id.to_string(),
                    )])
                    .order_by(kudos::timestamp::order(direction))
                    .with(kudos::to::fetch())
                    .take(amount)
                    .exec()
                    .await;

                match db_result {
                    Ok(kudos) => {
                        response
                            .push("Here is yours ")
                            .push(amount)
                            .push(if let OptionChoice::First = choice {
                                " earliest "
                            } else {
                                " latest "
                            })
                            .push("kudos:\n");

                        if kudos.len() == 0 {
                            response
                                .push("Looks like you don't like anyone.\n")
                                .push_italic_line_safe("Yuck, that's lonely");
                        } else {
                            response.push(format!(
                                "| {:<30} | {:<30} | {:<30} |\n",
                                "to", "message", "timestamp"
                            ));

                            kudos.iter().for_each(|kudos| {
                                if let Some(to) = &kudos.to {
                                    response.push(format!(
                                        "| {:<30} | {:<30} | {:<30} |\n",
                                        &to.username,
                                        &kudos.message,
                                        kudos.timestamp.to_string()
                                    ));
                                }
                            });
                        }
                    }
                    Err(why) => {
                        response
                            .push("Got some db error for you: \n")
                            .push_italic_line_safe(why.to_string());
                    }
                }
            }
            None => {
                response
                    .push("Wrong arguments, buddy.\n")
                    .push_italic_line_safe("Try again ?");
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
            .description("get received kudos".to_string())
            .create_option(|option| {
                option
                    .kind(CommandOptionType::String)
                    .name("order")
                    .description("either last or first")
                    .add_string_choice("last", OptionChoice::Last)
                    .add_string_choice("first", OptionChoice::First)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .kind(CommandOptionType::Integer)
                    .name("amount")
                    .description("max records to show")
                    .min_int_value(1)
                    .max_int_value(20)
                    .required(true)
            })
    }
}
