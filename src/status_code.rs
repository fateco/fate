macro_rules! define_error_macro {
    ( $name:ident, $msg:expr, $code:expr ) => {
        macro_rules! $name {
            () => {
                Response::error($msg, $code)
            };
        }
    };
}

define_error_macro!(bad_request, "Bad Request", 400);
define_error_macro!(unauthorized, "Unauthorized", 401);
define_error_macro!(not_found, "Not Found", 404);
define_error_macro!(method_not_allowed, "Method Not Allowed", 405);
define_error_macro!(internal_error, "Internal Server Error", 500);
