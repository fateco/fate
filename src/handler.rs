use futures::future::LocalBoxFuture;
use twilight_model::application::interaction::Interaction;
use worker::{Env, Response, Result};

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

pub fn get_handler(id: &str) -> Option<&'static Handler> {
    inventory::iter::<Handler>.into_iter().find(|handler| {
        let HandlerIds { auto, custom } = handler.0;
        auto == id || custom.is_some_and(|s| s == id)
    })
}
