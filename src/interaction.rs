use ed25519_dalek::Verifier;
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use worker::{Env, Method, Request, Response};

use crate::validate::{msg, public_key, signature};

pub async fn get_interaction(
    mut req: Request,
    env: &Env,
) -> Result<Interaction, worker::Result<Response>> {
    if req.method() != Method::Post {
        return Err(method_not_allowed!());
    }

    let body_bytes = req.bytes().await.map_err(|_| internal_error!())?;
    let headers = req.headers();
    public_key(env)
        .map_err(|_| internal_error!())?
        .verify(
            &msg(&body_bytes, headers).map_err(|_| bad_request!())?,
            &signature(headers).map_err(|_| bad_request!())?,
        )
        .map_err(|_| unauthorized!())?;

    serde_json::from_slice(&body_bytes).map_err(|_| bad_request!())
}

#[derive(Debug)]
pub enum InteractionId<'a> {
    Comand(&'a str),
    Other(&'a str),
}

pub fn interaction_id(interaction: &'_ Interaction) -> Option<InteractionId<'_>> {
    interaction.data.as_ref().and_then(|data| match data {
        InteractionData::ApplicationCommand(data) => Some(InteractionId::Comand(&data.name)),
        InteractionData::MessageComponent(data) => Some(InteractionId::Other(&data.custom_id)),
        InteractionData::ModalSubmit(data) => Some(InteractionId::Other(&data.custom_id)),
        _ => None,
    })
}

pub fn interaction_is_ping(interaction: &Interaction) -> Option<worker::Result<Response>> {
    matches!(interaction.kind, InteractionType::Ping).then(|| {
        Response::from_json(&InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        })
    })
}
