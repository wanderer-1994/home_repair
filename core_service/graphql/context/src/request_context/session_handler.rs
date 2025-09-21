use crate::RequestContext;
use account_service_server::{InitiateOrRenewSession, SessionAndCsrfToken};
use actor_auth::Session;
use async_graphql::Context;
use cookie::Cookie;
use error::Result;
use http::header::SET_COOKIE;
use service_http::{ACCESS_TOKEN_COOKIE_KEY, CSRF_TOKEN_COOKIE_KEY};
use std::sync::Arc;

impl RequestContext {
    /// Set
    /// - new http session
    /// - new context session
    ///
    /// and returns the new context session
    pub async fn set_session(
        &self,
        ctx: &Context<'_>,
        InitiateOrRenewSession {
            session_token,
            session,
        }: InitiateOrRenewSession,
    ) -> Result<Arc<Session>> {
        let new_session_ctx = Arc::new(session);
        let mut session_ctx_updater = self.session_context.write().await;
        session_ctx_updater.replace(Arc::clone(&new_session_ctx));
        self.set_session_cookies(ctx, session_token);

        Ok(new_session_ctx)
    }

    pub fn set_session_cookies(&self, ctx: &Context<'_>, token: SessionAndCsrfToken) {
        let jwt_cookie = Cookie::build((ACCESS_TOKEN_COOKIE_KEY, token.session_token))
            .secure(self.cookie_config.use_https)
            .same_site(self.cookie_config.same_site)
            .domain(&self.cookie_config.cookie_domain)
            .http_only(true)
            .build()
            .to_string();

        let csrf_cookie = Cookie::build((CSRF_TOKEN_COOKIE_KEY, token.csrf_token))
            .secure(self.cookie_config.use_https)
            .same_site(self.cookie_config.same_site)
            .domain(&self.cookie_config.cookie_domain)
            .build()
            .to_string();

        ctx.append_http_header(SET_COOKIE, jwt_cookie);
        ctx.append_http_header(SET_COOKIE, csrf_cookie);
    }

    pub fn clear_session_cookies(&self, ctx: &Context<'_>) {
        let zero = cookie::time::Duration::seconds(0);

        let jwt_cookie = Cookie::build((ACCESS_TOKEN_COOKIE_KEY, ""))
            .secure(self.cookie_config.use_https)
            .http_only(true)
            .domain(&self.cookie_config.cookie_domain)
            .max_age(zero)
            .build()
            .to_string();
        let csrf_cookie = Cookie::build((CSRF_TOKEN_COOKIE_KEY, ""))
            .secure(self.cookie_config.use_https)
            .domain(&self.cookie_config.cookie_domain)
            .max_age(zero)
            .build()
            .to_string();

        ctx.append_http_header(SET_COOKIE, jwt_cookie);
        ctx.append_http_header(SET_COOKIE, csrf_cookie);
    }
}
