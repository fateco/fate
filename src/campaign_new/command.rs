use anyhow::Result;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::InteractionContextType,
};
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

use crate::{
    campaign_new::handler::{
        CAMPAIGN_NEW_COMMAND, CAMPAIGN_NEW_SET_DEFAULT, CAMPAIGN_NEW_SET_LANG,
        CAMPAIGN_NEW_SET_NAME,
    },
    translation::{Truncate, all_t},
};
use fate_internal_macro::app_command;

#[app_command]
pub fn campaign_new_slash() -> Result<Command> {
    Ok(CommandBuilder::new(
        CAMPAIGN_NEW_COMMAND,
        t!("campaign_new.description").c(100),
        CommandType::ChatInput,
    )
    .name_localizations(all_t("test").iter().map(|f| (f.0, f.1.d(32))))
    .description_localizations(all_t("test"))
    .option(
        StringBuilder::new(
            CAMPAIGN_NEW_SET_NAME,
            t!("campaign_new.name_description").c(100),
        )
        .name_localizations(all_t("test").iter().map(|f| (f.0, f.1.d(32))))
        .description_localizations(all_t("test"))
        .required(true)
        .min_length(1)
        .max_length(20),
    )
    .option(
        StringBuilder::new(
            CAMPAIGN_NEW_SET_LANG,
            t!("campaign_new.lang_description").c(100),
        )
        .name_localizations(all_t("test").iter().map(|f| (f.0, f.1.d(32))))
        .description_localizations(all_t("test"))
        .required(true)
        .choices([("en", "en")]),
    )
    .option(
        BooleanBuilder::new(
            CAMPAIGN_NEW_SET_DEFAULT,
            t!("campaign_new.use_default_skills_description").c(100),
        )
        .name_localizations(all_t("test").iter().map(|f| (f.0, f.1.d(32))))
        .description_localizations(all_t("campaign_new.use_default_skills_description"))
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
