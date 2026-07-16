use std::sync::Arc;

use domain::connection::ConnectionManagerPort;

use crate::dto::ConnectionResponse;
use crate::errors::ApplicationError;

/// Use case: list every connection currently tracked by the pool.
pub struct ListConnectionsUseCase {
    manager: Arc<dyn ConnectionManagerPort>,
}

impl ListConnectionsUseCase {
    pub fn new(manager: Arc<dyn ConnectionManagerPort>) -> Self {
        Self { manager }
    }

    pub async fn execute(&self) -> Result<Vec<ConnectionResponse>, ApplicationError> {
        let connections = self.manager.list().await?;
        Ok(connections.into_iter().map(Into::into).collect())
    }
}
