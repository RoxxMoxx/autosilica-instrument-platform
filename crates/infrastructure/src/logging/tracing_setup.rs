use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initializes structured JSON logging via `tracing`.
///
/// `level` is a `tracing_subscriber::EnvFilter` directive (e.g.
/// `"info"`, `"debug"`, `"autosilica_server=debug,tower_http=info"`).
/// The `RUST_LOG` environment variable, if set, takes precedence.
pub fn init_tracing(level: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().json().with_target(true))
        .init();
}
