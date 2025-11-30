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
                TextInputBuilder::new("skills")
                    .paragraph()
                    .min_length(1)
                    .max_length(100)
                    .placeholder("placeholder")
                    .value("value")
                    .build()
                    .into(),
            )
            .response(SKILLS)
    }
}

mod handler {
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    use crate::{
        campaign_new::_1_1_skills::response::modal_skills, interaction_data::InteractionDataHelper,
        response::bad_request,
    };
    use fate_internal_macro::handler;

    #[handler]
    pub async fn skills(interaction: Interaction, env: Env) -> Result<Response> {
        // let Some((user_id, username)) = interaction.get_user() else {
        //     return bad_request();
        // };

        // todo!()
        modal_skills()
    }
}
