use std::sync::Arc;

use domain::entities::Instrument;
use domain::repositories::InstrumentRepository;

use crate::dto::{InstrumentResponse, RegisterInstrumentRequest};
use crate::errors::ApplicationError;

/// Use case: register a new instrument in the platform.
pub struct RegisterInstrumentUseCase {
    repository: Arc<dyn InstrumentRepository>,
}

impl RegisterInstrumentUseCase {
    pub fn new(repository: Arc<dyn InstrumentRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        request: RegisterInstrumentRequest,
    ) -> Result<InstrumentResponse, ApplicationError> {
        if request.name.trim().is_empty() {
            return Err(ApplicationError::InvalidInput("name must not be empty".into()));
        }

        let instrument = Instrument::new(request.name, request.model);
        let saved = self.repository.save(instrument).await?;
        Ok(saved.into())
    }
}
