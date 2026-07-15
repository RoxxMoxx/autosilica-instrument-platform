use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;

use domain::entities::Instrument;
use domain::errors::DomainError;
use domain::repositories::InstrumentRepository;
use domain::value_objects::InstrumentId;

/// In-memory `InstrumentRepository` adapter.
///
/// Intended as a development/testing default and as a reference
/// implementation of the `InstrumentRepository` port. A future
/// persistent adapter (e.g. Postgres) can be dropped in without any
/// change to `domain` or `application`, thanks to Dependency
/// Inversion.
#[derive(Default)]
pub struct InMemoryInstrumentRepository {
    store: RwLock<HashMap<InstrumentId, Instrument>>,
}

impl InMemoryInstrumentRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl InstrumentRepository for InMemoryInstrumentRepository {
    async fn list(&self) -> Result<Vec<Instrument>, DomainError> {
        let store = self
            .store
            .read()
            .map_err(|_| DomainError::Validation("repository lock poisoned".into()))?;
        Ok(store.values().cloned().collect())
    }

    async fn get(&self, id: InstrumentId) -> Result<Instrument, DomainError> {
        let store = self
            .store
            .read()
            .map_err(|_| DomainError::Validation("repository lock poisoned".into()))?;
        store
            .get(&id)
            .cloned()
            .ok_or_else(|| DomainError::NotFound(id.to_string()))
    }

    async fn save(&self, instrument: Instrument) -> Result<Instrument, DomainError> {
        let mut store = self
            .store
            .write()
            .map_err(|_| DomainError::Validation("repository lock poisoned".into()))?;
        store.insert(instrument.id, instrument.clone());
        Ok(instrument)
    }
}
