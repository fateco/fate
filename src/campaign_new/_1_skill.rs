pub mod response {
    use crate::{
        campaign_new::_1_skill::handler::SKILLS,
        modal::{Modal, input::TextInputBuilder},
    };
    use worker::{Response, Result};

    pub fn modal_skill(lang: &str, value: &str) -> Result<Response> {
        Modal::new("Skills")
            .row(
                "Skills",
                Some("Skills"),
                TextInputBuilder::new("id")
                    .paragraph()
                    .min_length(37)
                    .max_length(4000) //524)
                    .placeholder("placeholder")
                    .value(value)
                    .build()
                    .into(),
            )
            .response(SKILLS)
    }
}

mod handler {
    use twilight_model::{
        application::interaction::Interaction,
        http::interaction::{
            InteractionResponse, InteractionResponseData, InteractionResponseType,
        },
    };
    use worker::{Env, Response, Result};

    use crate::{
        interaction_data::InteractionDataHelper,
        modal::input::TextInputBuilder,
        response::{bad_request, response},
    };
    use fate_internal_macro::handler;

    #[handler]
    pub async fn skills(interaction: Interaction, env: Env) -> Result<Response> {
        let Some((user_id, username, locale)) = interaction.get_user() else {
            return bad_request();
        };

        todo!()
    }
}
