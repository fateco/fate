use futures::future::LocalBoxFuture;
use twilight_model::application::interaction::Interaction;
use worker::{Env, Response, Result};

use crate::interaction::InteractionId;

#[derive(Debug)]
pub struct HandlerIds {
    pub auto: &'static str,
    pub custom: Option<&'static str>,
}

#[derive(Debug)]
pub struct Handler(
    pub HandlerIds,
    pub fn(Interaction, Env) -> LocalBoxFuture<'static, Result<Response>>,
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
            InteractionId::Comand(id) => self.custom.is_some_and(|v| &v == id),
            InteractionId::Other(id) => &self.auto == id,
        }
    }
}
