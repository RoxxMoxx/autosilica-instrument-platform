use std::sync::Arc;

use domain::connection::ConnectionManagerPort;

use crate::dto::ConnectionResponse;
use crate::errors::ApplicationError;

/// Use case: connect to a VISA resource, reusing an existing pooled
/// connection when one is already open.
pub struct ConnectInstrumentUseCase {
    manager: Arc<dyn ConnectionManagerPort>,
}

impl ConnectInstrumentUseCase {
    pub fn new(manager: Arc<dyn ConnectionManagerPort>) -> Self {
        Self { manager }
    }

    pub async fn execute(&self, resource: &str) -> Result<ConnectionResponse, ApplicationError> {
        if resource.trim().is_empty() {
            return Err(ApplicationError::InvalidInput(
                "resource must not be empty".into(),
            ));
        }

        let connection = self.manager.connect(resource).await?;
        Ok(connection.into())
    }
}
