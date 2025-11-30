mod response {}

mod handler {
    use twilight_model::application::interaction::Interaction;
    use worker::{Env, Response, Result};

    use crate::{interaction_data::InteractionDataHelper, response::bad_request};
    use fate_internal_macro::handler;

    #[handler]
    pub async fn physical(interaction: Interaction, env: Env) -> Result<Response> {
        let Some((user_id, username)) = interaction.get_user() else {
            return bad_request();
        };

        todo!()
    }
}
