use serenity::utils::MessageBuilder;

pub fn unknown_role_message(why: String) -> String {
    String::from(format!(
        "Sorry, I can't process what species are you? (Unknown Role) [{:?}]",
        why
    ))
}

pub fn me_command_response(username: &String, reputation: &String) -> String {
    MessageBuilder::new()
        .push("User ")
        .push_bold_safe(username)
        .push(" has the following reputation of ")
        .push_bold_safe(reputation)
        .build()
}
