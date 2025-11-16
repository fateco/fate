use twilight_model::application::interaction::Interaction;
use worker::{Env, Response, Result};

use crate::interaction_data::InteractionDataHelper;
use fate_internal_macro::handler;

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
    let (Some(a), Some(b)) = (
        interaction.get_command_v_str("option_name"),
        interaction.get_command_v_str("option_name"),
    ) else {
        return bad_request!();
    };
    bad_request!()
}
