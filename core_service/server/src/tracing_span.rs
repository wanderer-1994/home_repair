use core_service_graphql_context::RequestContext;
use tracing::info_span;

pub async fn make_user_context_span(context: &RequestContext) -> tracing::Span {
    let maybe_session_context = context.try_session_context().await.ok();
    let ip_address = context.remote_addr.ip().to_string();
    if let Some(session) = maybe_session_context {
        let actor = format!("{:?}", session.actor_type);
        return info_span!("Authenticated GraphQL request", actor, ip_address);
    }
    info_span!("Anonymous GraphQL request", ip_address)
}
