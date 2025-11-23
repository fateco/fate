use anyhow::Result;
use twilight_model::application::command::Command;
use twilight_util::builder::command::CommandBuilder;

use crate::{
    campaign_new::{command::_1_lang::lang, handler::CAMPAIGN_NEW_CUSTOMNAME},
    slash::CommandLocalize,
    translation::{Truncate, all_t},
};
use fate_internal_macro::app_command;

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::init_localize(
        CAMPAIGN_NEW_CUSTOMNAME,
        t!("campaign_new.description").c(100),
        all_t("campaign_new.command", 32),
        all_t("campaign_new.description", 100),
    )
    .option(lang())
    .validate()?
    .build())
}
