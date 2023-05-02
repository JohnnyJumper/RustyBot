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
