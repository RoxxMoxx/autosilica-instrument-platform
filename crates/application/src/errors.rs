use thiserror::Error;

use domain::errors::DomainError;

/// Errors surfaced by the application layer. Wraps domain errors and
/// adds application-specific failure modes (e.g. bad input that never
/// reaches the domain).
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}
