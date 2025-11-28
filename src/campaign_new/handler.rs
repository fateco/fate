use twilight_model::{
    application::interaction::Interaction, http::interaction::InteractionResponseType,
};
use worker::{Env, Response, Result};

use crate::{
    campaign_new::command::_1_lang::CAMPAIGN_NEW_SET_LANG,
    interaction_data::InteractionDataHelper,
    modal::{self, Modal, input::TextInputBuilder},
    response::bad_request,
};
use fate_internal_macro::handler;

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
    // let (Some(user_id), Some(lang)) = (
    //     interaction.get_user_id(),
    //     interaction.get_command_v_str(CAMPAIGN_NEW_SET_LANG),
    // ) else {
    //     return bad_request();
    // };
    Modal::new("New Campaign")
        .row(
            "Skills",
            "Skills",
            TextInputBuilder::new("skills")
                .paragraph()
                .min_length(1)
                .max_length(100)
                .placeholder("placeholder")
                .value("value")
                .build()
                .into(),
        )
        .response("TODO")
}
