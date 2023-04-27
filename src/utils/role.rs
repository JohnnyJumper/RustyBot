use lazy_static::lazy_static;
use serenity::http::Http;
use serenity::model::prelude::{GuildId, RoleId};
use serenity::model::user::User;
use serenity::Error;
use std::env;
use std::sync::Arc;

pub enum UserRole {
    User,
    Admin,
    Bot,
    Unknown(String),
}

lazy_static! {
    pub static ref BOT_ADMIN_ROLE: RoleId = match env::var("BOT_ADMIN_ROLE") {
        Ok(val) =>
            RoleId(val.parse().unwrap_or_else(|_err| panic!(
                "BOT_ADMIN_ROLE enviroment variable is not a number"
            ))),
        Err(_why) => panic!("BOT_ADMIN_ROLE enviroment variable is not set"),
    };
    pub static ref BOT_ROLE: RoleId = match env::var("BOT_ROLE") {
        Ok(val) => RoleId(
            val.parse()
                .unwrap_or_else(|_err| panic!("BOT_ROLE enviroment variable is not a number"))
        ),
        Err(_why) => panic!("BOT_ROLE enviroment variable is not set"),
    };
    pub static ref GUILD_ID: GuildId = match env::var("GUILD_ID") {
        Ok(val) => GuildId(
            val.parse()
                .unwrap_or_else(|_err| panic!("GUILD_ID enviroment variable is not a number"))
        ),
        Err(_why) => panic!("GUILD_ID enviroment variable is not set"),
    };
}

pub async fn identify_role(user: &User, http: &Arc<Http>) -> Result<UserRole, Error> {
    let guild_id = &GUILD_ID;
    let admin_role_id = &BOT_ADMIN_ROLE;
    let bot_role_id = &BOT_ROLE;

    if user
        .has_role(http, *guild_id.as_u64(), *admin_role_id.as_u64())
        .await?
    {
        return Ok(UserRole::Admin);
    } else if user
        .has_role(http, *guild_id.as_u64(), *bot_role_id.as_u64())
        .await?
    {
        return Ok(UserRole::Bot);
    } else {
        return Ok(UserRole::User);
    }
}
