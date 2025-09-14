//! Centralize crate defining how FE communicating with BE

/// Key for the JWT cookie.
pub const ACCESS_TOKEN_COOKIE_KEY: &str = "access_token";

/// Key for the CSRF token
pub const CSRF_TOKEN_COOKIE_KEY: &str = "xsrf";

/// Header key for the CSRF token that is provided by the client during requests.
/// See <https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html>
pub const CSRF_TOKEN_HEADER_KEY: &str = "x-xsrf";
