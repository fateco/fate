use twilight_model::application::interaction::{Interaction, InteractionData};
use worker::{Env, Response, Result};

use fate_internal_macro::handler;

pub const CAMPAIGN_NEW_SET_NAME: &str = "";
pub const CAMPAIGN_NEW_SET_LANG: &str = "";
pub const CAMPAIGN_NEW_SET_DEFAULT: &str = "";

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
    if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        //data.options
        internal_error!()
    } else {
        bad_request!()
    }
}
