use thiserror::Error;

/// Errors that can be raised by domain logic itself (invariants,
/// business rule violations), independent of any technical concern.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("entity not found: {0}")]
    NotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),
}
