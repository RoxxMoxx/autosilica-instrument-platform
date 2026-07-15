use thiserror::Error;

/// Errors originating from infrastructure concerns (configuration,
/// I/O, etc.), kept distinct from domain/application errors.
#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
