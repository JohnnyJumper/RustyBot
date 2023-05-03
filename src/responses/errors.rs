use serenity::utils::MessageBuilder;

pub fn only_admin_message() -> String {
    MessageBuilder::new()
        .push("Only bot overseers allowed to use this command\n")
        .push_bold_line_safe("This action will be noted")
        .build()
}

pub fn not_registered_message(name: &String) -> String {
    MessageBuilder::new()
        .push("User ")
        .push_bold_line_safe(name)
        .push(" is not registered, talk with admins")
        .build()
}

pub fn user_not_found_message(name: &String, why: String) -> String {
    MessageBuilder::new()
        .push("Error looking into db for user ")
        .push_bold_line_safe(name)
        .push(" : ")
        .push(why)
        .build()
}

pub fn error_retriving_guild_members_message(why: String) -> String {
    MessageBuilder::new()
        .push("Error retriving guild members: ")
        .push_italic_safe(why)
        .build()
}

pub fn general_unknown_error_message(why: String) -> String {
    MessageBuilder::new()
        .push("Something went wrong with the following error: \n")
        .push_bold_line_safe(why)
        .build()
}

pub fn only_one_kudos_error_message() -> String {
    MessageBuilder::new()
        .push("Only one kudos per day \n")
        .push_bold_line_safe("Try again in tomorrow")
        .build()
}

pub fn unknown_user_error_message() -> String {
    MessageBuilder::new()
        .push("Failed to understand the receiver of kudos (unknown user)")
        .build()
}

pub fn cannot_increase_your_own_reputation_message() -> String {
    MessageBuilder::new()
        .push("I see what you are trying to do here\n")
        .push_bold_line_safe("This action will be noted!")
        .build()
}
