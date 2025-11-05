use futures::future::BoxFuture;
use twilight_model::application::interaction::Interaction;
use worker::{Env, Response, Result};

use crate::interaction::InteractionId;

#[derive(Debug)]
pub struct HandlerIds {
    pub command: Option<&'static str>,
    pub common: &'static str,
}

#[derive(Debug)]
pub struct Handler(
    pub HandlerIds,
    pub fn(Interaction, Env) -> BoxFuture<'static, Result<Response>>,
);

inventory::collect!(Handler);

pub fn get_handler(id: &InteractionId) -> Option<&'static Handler> {
    inventory::iter::<Handler>
        .into_iter()
        .find(|handler| &handler.0 == id)
}

impl PartialEq<InteractionId<'_>> for HandlerIds {
    fn eq(&self, other: &InteractionId) -> bool {
        match other {
            InteractionId::Comand(id) => self.command.is_some_and(|v| &v == id),
            InteractionId::Common(id) => &self.common == id,
        }
    }
}
