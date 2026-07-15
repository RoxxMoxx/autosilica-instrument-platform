use std::sync::Arc;

use domain::repositories::InstrumentRepository;

use crate::dto::InstrumentResponse;
use crate::errors::ApplicationError;

/// Use case: list all known instruments.
pub struct ListInstrumentsUseCase {
    repository: Arc<dyn InstrumentRepository>,
}

impl ListInstrumentsUseCase {
    pub fn new(repository: Arc<dyn InstrumentRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<InstrumentResponse>, ApplicationError> {
        let instruments = self.repository.list().await?;
        Ok(instruments.into_iter().map(Into::into).collect())
    }
}
