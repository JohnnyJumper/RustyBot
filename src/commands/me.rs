use serenity::builder::CreateApplicationCommand;
use serenity::model::user::User;
use serenity::utils::MessageBuilder;

pub fn run(user: &User) -> String {
    // prisma query for reputation
    MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&user.name)
        .push(" has the following reputation of ")
        .push_bold_safe(10)
        .build()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("me").description("show my reputation")
}
