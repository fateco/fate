use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;
use worker::{Env, Response, Result};

use fate_internal_macro::handler;

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
    if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        Response::from_json(&InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseDataBuilder::new().content("Hi!").build()),
        })
    } else {
        bad_request!()
    }
}

fn a(b: InteractionData) {}
