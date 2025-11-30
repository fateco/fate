mod slash {
    use anyhow::Result;
    use twilight_model::application::command::Command;
    use twilight_util::builder::command::{CommandBuilder, StringBuilder};

    use crate::{
        campaign_new::_0_slash::handler::CAMPAIGN_NEW_CUSTOMNAME,
        slash::CommandLocalize,
        translation::{Truncate, all_t, langs},
    };
    use fate_internal_macro::app_command;

    pub const LANG: &str = "language";

    #[app_command]
    pub fn campaign_new_slash() -> Result<Command> {
        let campaign_new = CommandBuilder::init_localize(
            CAMPAIGN_NEW_CUSTOMNAME,
            t!("campaign_new.description").c(100),
            all_t("campaign_new.command", 32),
            all_t("campaign_new.description", 100),
        );

        let language = StringBuilder::init_localize(
            LANG,
            t!("campaign_new.language_description").c(100),
            all_t("campaign_new.language", 32),
            all_t("campaign_new.language_description", 100),
        )
        .required(true)
        .choices(langs());

        Ok(campaign_new.option(language).validate()?.build())
    }
}

mod handler {
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    use crate::{
        campaign_new::{_0_slash::slash::LANG, _1_1_skills::response::modal_skills},
        interaction_data::InteractionDataHelper,
        response::bad_request,
    };
    use fate_internal_macro::handler;

    #[handler("new-campaign")]
    pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
        let (Some((user_id, username)), Some(lang)) =
            (interaction.get_user(), interaction.get_command_v_str(LANG))
        else {
            return bad_request();
        };

        modal_skills()
    }
}
