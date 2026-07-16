use std::sync::Arc;

use domain::connection::ConnectionManagerPort;

use crate::dto::ConnectionResponse;
use crate::errors::ApplicationError;

/// Use case: disconnect a pooled connection to a VISA resource.
pub struct DisconnectInstrumentUseCase {
    manager: Arc<dyn ConnectionManagerPort>,
}

impl DisconnectInstrumentUseCase {
    pub fn new(manager: Arc<dyn ConnectionManagerPort>) -> Self {
        Self { manager }
    }

    pub async fn execute(&self, resource: &str) -> Result<ConnectionResponse, ApplicationError> {
        if resource.trim().is_empty() {
            return Err(ApplicationError::InvalidInput(
                "resource must not be empty".into(),
            ));
        }

        let connection = self.manager.disconnect(resource).await?;
        Ok(connection.into())
    }
}
