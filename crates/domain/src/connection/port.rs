use async_trait::async_trait;

use crate::errors::DomainError;

use super::Connection;

/// Port for the instrument connection pool.
///
/// Implemented by an infrastructure adapter that holds real,
/// persistent connections (e.g. open VISA sessions) and reuses them
/// across calls rather than opening a new one every time.
#[async_trait]
pub trait ConnectionManagerPort: Send + Sync {
    /// Connects to `resource`, reusing an already-open connection if
    /// one exists. Returns the resulting [`Connection`] record even
    /// when the underlying attempt failed (state `Error` in that
    /// case) — a failed connect is a normal, representable outcome,
    /// not an application-level error.
    async fn connect(&self, resource: &str) -> Result<Connection, DomainError>;

    /// Disconnects `resource`, closing its underlying connection if
    /// open. Fails with `DomainError::NotFound` if `resource` is not
    /// currently tracked by the pool.
    async fn disconnect(&self, resource: &str) -> Result<Connection, DomainError>;

    /// Lists every connection currently tracked by the pool.
    async fn list(&self) -> Result<Vec<Connection>, DomainError>;
}
