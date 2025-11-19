use twilight_model::application::interaction::Interaction;
use worker::{Env, Response, Result};

use crate::{
    campaign_new::command::_1_name::CAMPAIGN_NEW_SET_NAME,
    campaign_new::command::_2_lang::CAMPAIGN_NEW_SET_LANG, database::campaign::new_campaign,
    interaction_data::InteractionDataHelper,
};
use fate_internal_macro::handler;

#[handler("new-campaign")]
pub async fn campaign_new(interaction: Interaction, env: Env) -> Result<Response> {
    let (Some(name), Some(lang)) = (
        interaction.get_command_v_str(CAMPAIGN_NEW_SET_NAME),
        interaction.get_command_v_str(CAMPAIGN_NEW_SET_LANG),
    ) else {
        return bad_request!();
    };
    let d1 = env.d1("fate_db")?;
    let a = new_campaign(&d1, name, lang, 1).await?;
    a.run().await?;
    // Ok(Response::from_json(&InteractionResponse {
    //     kind: InteractionResponseType::Pong,
    //     data: None,
    // }))
    todo!()
}
