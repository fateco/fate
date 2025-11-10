use anyhow::Result;
use twilight_model::application::command::Command;
use twilight_util::builder::command::CommandBuilder;

use crate::{
    campaign_new::{
        command::{_1_name::name, _2_lang::lang, _3_is_defualt_skills::is_defualt_skills},
        handler::CAMPAIGN_NEW_COMMAND,
    },
    slash::CommandLocalize,
    translation::{Truncate, all_t},
};
use fate_internal_macro::app_command;

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::init_localize(
        CAMPAIGN_NEW_COMMAND,
        t!("campaign_new.description").c(100),
        all_t("campaign_new.command", 32),
        all_t("campaign_new.description", 100),
    )
    .option(name())
    .option(lang())
    .option(is_defualt_skills())
    .validate()?
    .build())
}
