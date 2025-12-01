mod command {
    use crate::{
        campaign_new::_0_slash::handler::CAMPAIGN_NEW_CUSTOMNAME,
        slash::CommandLocalize,
        translation::{Truncate, all_t, langs},
    };
    use anyhow::Result;
    use fate_internal_macro::app_command;
    use twilight_model::application::command::Command;
    use twilight_util::builder::command::{CommandBuilder, StringBuilder};

    pub const LANG: &str = "language";
    pub const NAME: &str = "name";

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

        let name = StringBuilder::init_localize(
            NAME,
            t!("campaign_new.name_description").c(100),
            all_t("campaign_new.name", 32),
            all_t("campaign_new.name_description", 100),
        )
        .required(true)
        .min_length(3)
        .max_length(20);

        Ok(campaign_new
            .option(language)
            .option(name)
            .validate()?
            .build())
    }
}

mod handler {
    use crate::{
        campaign_new::{
            _0_slash::command::{LANG, NAME},
            _1_skill::response::modal_skill,
        },
        interaction_data::InteractionDataHelper,
        response::bad_request,
    };
    use fate_internal_macro::handler;
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    #[handler("new-campaign")]
    pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
        let (Some((user_id, username, locale)), Some(c_lang), Some(c_name)) = (
            interaction.get_user(),
            interaction.get_command_v_str(LANG),
            interaction.get_command_v_str(NAME),
        ) else {
            return bad_request();
        };

        modal_skill(
            locale,
            &[
                t!("default_skill.academics", locale = c_lang),
                t!("default_skill.athletics", locale = c_lang),
                t!("default_skill.burglary", locale = c_lang),
                t!("default_skill.contacts", locale = c_lang),
                t!("default_skill.crafts", locale = c_lang),
                t!("default_skill.deceive", locale = c_lang),
                t!("default_skill.drive", locale = c_lang),
                t!("default_skill.empathy", locale = c_lang),
                t!("default_skill.fight", locale = c_lang),
                t!("default_skill.investigate", locale = c_lang),
                t!("default_skill.lore", locale = c_lang),
                t!("default_skill.notice", locale = c_lang),
                t!("default_skill.physique", locale = c_lang),
                t!("default_skill.provoke", locale = c_lang),
                t!("default_skill.rapport", locale = c_lang),
                t!("default_skill.resources", locale = c_lang),
                t!("default_skill.shoot", locale = c_lang),
                t!("default_skill.stealth", locale = c_lang),
                t!("default_skill.will", locale = c_lang),
            ]
            .join("\n"),
        )
    }
}

// pub_msg(
//     "",
//     vec![W
//         ActionRowBuilder::new()
//             .component(
//                 SelectMenuBuilder::new("custom_id", SelectMenuType::Text)
//                     .option(
//                         SelectMenuOptionBuilder::new("dsds", "dsdddd")
//                             .emoji(EmojiReactionType::Unicode {
//                                 name: "👩🏾‍❤️‍💋‍👨🏻".into()
//                             })
//                             .build(),
//                     )
//                     .build(),
//             )
//             .build()
//             .into(),
//     ],
// )
