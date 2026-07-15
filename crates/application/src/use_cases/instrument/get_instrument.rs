use std::sync::Arc;

use domain::repositories::InstrumentRepository;
use domain::value_objects::InstrumentId;
use uuid::Uuid;

use crate::dto::InstrumentResponse;
use crate::errors::ApplicationError;

/// Use case: fetch a single instrument by its identifier.
pub struct GetInstrumentUseCase {
    repository: Arc<dyn InstrumentRepository>,
}

impl GetInstrumentUseCase {
    pub fn new(repository: Arc<dyn InstrumentRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, raw_id: &str) -> Result<InstrumentResponse, ApplicationError> {
        let uuid = Uuid::parse_str(raw_id)
            .map_err(|_| ApplicationError::InvalidInput("invalid instrument id".into()))?;

        let instrument = self.repository.get(InstrumentId(uuid)).await?;
        Ok(instrument.into())
    }
}
