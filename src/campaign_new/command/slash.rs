use anyhow::Result;
use std::borrow::Cow;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::InteractionContextType,
};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    campaign_new::{
        command::{is_defualt_skills::is_defualt_skills, lang::lang, name::name},
        handler::CAMPAIGN_NEW_COMMAND,
    },
    translation::{Truncate, all_t},
};
use fate_internal_macro::app_command;

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::new(
        Cow::Borrowed(CAMPAIGN_NEW_COMMAND).cut_n_fill(32),
        t!("campaign_new.description").c(100),
        CommandType::ChatInput,
    )
    .contexts([
        InteractionContextType::Guild,
        InteractionContextType::BotDm,
        InteractionContextType::PrivateChannel,
    ])
    .name_localizations(
        all_t("campaign_new.command")
            .iter()
            .map(|f| (f.0, f.1.cut_n_fill(32))),
    )
    .description_localizations(all_t("campaign_new.description"))
    .option(name())
    .option(lang())
    .option(is_defualt_skills())
    .validate()?
    .build())
}
