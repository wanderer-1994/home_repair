use account_service_server::{
    AccountService, SessionAndCsrfToken, ValidateSessionTokenRequest, ValidateSessionTokenStrategy,
};
use actor_auth::Session;
use headers::{Cookie, HeaderMap, HeaderMapExt};
use service_http::{ACCESS_TOKEN_COOKIE_KEY, CSRF_TOKEN_HEADER_KEY};

/// Extract user session from http request headers
pub(crate) async fn extract_session(
    header_map: &HeaderMap,
    account_service_client: &AccountService,
) -> Option<Session> {
    let cookies = header_map.typed_get::<Cookie>();

    let session_token = cookies.as_ref().and_then(|cookies| {
        cookies
            .get(ACCESS_TOKEN_COOKIE_KEY)
            .filter(|s| !s.is_empty())
    })?;

    let csrf_token = header_map
        .get(CSRF_TOKEN_HEADER_KEY)
        .and_then(|value| value.to_str().ok().filter(|s| !s.is_empty()))?;

    account_service_client
        .validate_session_token(ValidateSessionTokenRequest {
            strategy: ValidateSessionTokenStrategy::CsrfProtectionRequired(SessionAndCsrfToken {
                session_token: session_token.into(),
                csrf_token: csrf_token.into(),
            }),
        })
        .await
        .map(|r| r.session)
        .inspect_err(|e| tracing::error!(?e, "Failed to verify access token"))
        .ok()
        .flatten()
}
