use anyhow::Result;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::InteractionContextType,
};
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

use crate::{campaign_new::handler::CAMPAIGN_NEW_COMMAND, translation::Truncate};
use fate_internal_macro::app_command;

pub const CAMPAIGN_NEW_SET_NAME: &str = "";
pub const CAMPAIGN_NEW_SET_LANG: &str = "";
pub const CAMPAIGN_NEW_SET_DEFAULT: &str = "";

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::new(
        CAMPAIGN_NEW_COMMAND,
        t!("campaign_new.description").c(100),
        CommandType::ChatInput,
    )
    .option(
        StringBuilder::new(
            CAMPAIGN_NEW_SET_NAME,
            t!("campaign_new.name_description").c(100),
        )
        .required(true)
        .min_length(1)
        .max_length(20),
    )
    .option(
        StringBuilder::new(
            CAMPAIGN_NEW_SET_LANG,
            t!("campaign_new.lang_description").c(100),
        )
        .required(true)
        .choices([("en", "en")]),
    )
    .option(
        BooleanBuilder::new(
            CAMPAIGN_NEW_SET_DEFAULT,
            t!("campaign_new.use_default_skills_description").c(100),
        )
        .required(true),
    )
    .contexts([
        InteractionContextType::Guild,
        InteractionContextType::BotDm,
        InteractionContextType::PrivateChannel,
    ])
    .validate()?
    .build())
}
