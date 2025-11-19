#![allow(clippy::future_not_send)]

#[macro_use]
extern crate rust_i18n;

pub use slash::get_commands;

#[macro_use]
mod status_code;

mod handler;
mod interaction;
mod interaction_data;
mod slash;
mod translation;
mod validate;

mod database;

mod campaign_new;

use worker::{Context, Env, Request, Response, Result, event};

use crate::{
    handler::get_handler,
    interaction::{get_interaction, interaction_id, interaction_is_ping},
};

i18n!("translations", fallback = "en-US");

///  This is where the Cloudflare Worker starts working on a Discord interaction for the FATE app and sends a reply.
///
/// # Errors
///
/// If this function runs into any errors when working with a Worker, an error result will be returned.
#[event(fetch)]
pub async fn app(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let interaction = match get_interaction(req, &env).await {
        Ok(interaction) => interaction,
        Err(code) => return code,
    };

    if let Some(pong) = interaction_is_ping(&interaction) {
        return pong;
    }

    match interaction_id(&interaction).and_then(|id| get_handler(&id)) {
        Some(handler) => (handler.1)(interaction, env).await,
        None => not_found!(),
    }
}
