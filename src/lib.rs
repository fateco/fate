#![allow(clippy::future_not_send)]

#[macro_use]
extern crate rust_i18n;

pub use slash::get_commands;

mod handler;
mod interaction_data;
mod modal;
mod process;
mod response;
mod slash;
mod translation;

mod components;
mod database;

mod campaign_new;

use crate::{
    handler::get_handler,
    interaction_data::InteractionDataHelper,
    process::get_interaction,
    response::{not_found, pong},
};
use worker::{Context, Env, Request, Response, Result, event};

i18n!("translations", fallback = "en-US");

/// The main entry point for FATE. It handles every request from Discord, figures out what to do, and sends a reply.
///
/// # Errors
///
/// If this function runs into any errors when working with a Worker, a worker error will be returned.
#[event(fetch)]
pub async fn app(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let interaction = match get_interaction(req, &env).await {
        Ok(i) => i,
        Err(e) => return e,
    };

    let Some(id) = interaction.get_id() else {
        return pong();
    };

    let Some(handler) = get_handler(id) else {
        return not_found();
    };

    (handler.1)(interaction, env).await
}
