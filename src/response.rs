use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use worker::{Response, Result};

pub fn response(interaction_response: &InteractionResponse) -> Result<Response> {
    Response::from_json(interaction_response)
}

pub fn pong() -> Result<Response> {
    response(&InteractionResponse {
        kind: InteractionResponseType::Pong,
        data: None,
    })
}

pub mod msg {
    use twilight_model::{
        channel::message::{Component, MessageFlags},
        http::interaction::{
            InteractionResponse, InteractionResponseData, InteractionResponseType,
        },
    };
    use worker::{Response, Result};

    use crate::response::response;

    fn msg(
        custom_id: impl Into<String>,
        components: Vec<Component>,
        flags: MessageFlags,
    ) -> Result<Response> {
        response(&InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                custom_id: Some(custom_id.into()),
                components: Some(components),
                flags: Some(MessageFlags::IS_COMPONENTS_V2 | flags),
                ..Default::default()
            }),
        })
    }

    pub fn ephemeral(components: Vec<Component>, custom_id: impl Into<String>) -> Result<Response> {
        msg(custom_id, components, MessageFlags::EPHEMERAL)
    }

    pub fn pub_msg(components: Vec<Component>, custom_id: impl Into<String>) -> Result<Response> {
        msg(custom_id, components, MessageFlags::empty())
    }
}

macro_rules! define_error_macro {
    ( $name:ident, $msg:expr, $code:expr ) => {
        pub fn $name() -> Result<Response> {
            Response::error($msg, $code)
        }
    };
}

define_error_macro!(bad_request, "Bad Request", 400);
define_error_macro!(unauthorized, "Unauthorized", 401);
define_error_macro!(not_found, "Not Found", 404);
define_error_macro!(method_not_allowed, "Method Not Allowed", 405);
define_error_macro!(internal_error, "Internal Server Error", 500);
