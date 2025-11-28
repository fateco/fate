use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
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
