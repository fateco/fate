mod response {
    use worker::{Response, Result};

    pub fn msg_reuse() -> Result<Response> {
        todo!()
    }
}

mod handler {
    use crate::{interaction_data::InteractionDataHelper, response::bad_request};
    use fate_internal_macro::handler;
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    #[handler]
    pub async fn reuse(interaction: Interaction, env: Env) -> Result<Response> {
        let Some((user_id, username, locale)) = interaction.get_user() else {
            return bad_request();
        };

        todo!()
    }
}
