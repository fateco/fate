use anyhow::Result;
use twilight_model::application::command::Command;
use twilight_util::builder::command::CommandBuilder;

use crate::{
    campaign_new::{
        command::{_1_name::name, _2_lang::lang, _3_is_defualt_skills::is_defualt_skills},
        handler::CAMPAIGN_NEW_COMMAND,
    },
    slash::CommandLocalize,
    translation::all_t,
};
use fate_internal_macro::app_command;

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::init_localize(
        CAMPAIGN_NEW_COMMAND,
        all_t("campaign_new.name", 32),
        all_t("campaign_new.name_description", 100),
    )
    .option(name())
    .option(lang())
    .option(is_defualt_skills())
    .validate()?
    .build())
}
