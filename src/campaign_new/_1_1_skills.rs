pub mod response {
    use worker::{Response, Result};

    use crate::{
        campaign_new::_1_1_skills::handler::SKILLS,
        modal::{Modal, input::TextInputBuilder},
    };

    pub fn modal_skills() -> Result<Response> {
        Modal::new("Skills")
            .row(
                "Skills",
                Some("Skills"),
                TextInputBuilder::new("id")
                    .paragraph()
                    .min_length(55)
                    .max_length(3449)
                    .placeholder("placeholder")
                    .value("value")
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
        let Some((user_id, username)) = interaction.get_user() else {
            return bad_request();
        };

        todo!()
    }
}
