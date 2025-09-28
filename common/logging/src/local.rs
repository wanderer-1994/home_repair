use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Initialize tracing with a simple layer that logs to stdout
pub fn init_tracing_local() {
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}
