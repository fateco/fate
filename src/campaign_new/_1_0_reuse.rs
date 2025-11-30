mod response {
    use worker::{Response, Result};

    pub fn msg_reuse() -> Result<Response> {
        todo!()
    }
}

mod handler {
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    use crate::{interaction_data::InteractionDataHelper, response::bad_request};
    use fate_internal_macro::handler;

    #[handler]
    pub async fn reuse(interaction: Interaction, env: Env) -> Result<Response> {
        let Some((user_id, username)) = interaction.get_user() else {
            return bad_request();
        };

        todo!()
    }
}
