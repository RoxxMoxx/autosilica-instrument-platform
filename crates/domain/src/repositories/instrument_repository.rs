use async_trait::async_trait;

use crate::entities::Instrument;
use crate::errors::DomainError;
use crate::value_objects::InstrumentId;

/// Repository "port" for [`Instrument`] persistence.
///
/// This trait is defined in the domain layer and implemented by an
/// adapter in the `infrastructure` layer (Dependency Inversion
/// Principle: the domain does not depend on infrastructure, it is the
/// other way around).
#[async_trait]
pub trait InstrumentRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Instrument>, DomainError>;
    async fn get(&self, id: InstrumentId) -> Result<Instrument, DomainError>;
    async fn save(&self, instrument: Instrument) -> Result<Instrument, DomainError>;
}
