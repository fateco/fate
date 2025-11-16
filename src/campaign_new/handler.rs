use twilight_model::{
    application::interaction::{Interaction},
};
use worker::{Env, Response, Result};

use fate_internal_macro::handler;

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {


    bad_request!()
}
