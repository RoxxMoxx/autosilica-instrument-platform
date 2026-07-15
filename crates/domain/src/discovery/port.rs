use async_trait::async_trait;

use crate::errors::DomainError;

use super::DiscoveredInstrument;

/// Port for discovering instruments reachable over VISA (USB,
/// LAN/TCPIP, GPIB, Serial/ASRL, ...).
///
/// Implemented by an infrastructure adapter (e.g. one backed by
/// NI-VISA). The domain layer has no knowledge of VISA itself —
/// following Dependency Inversion, this trait is the boundary the
/// outer layer must satisfy.
#[async_trait]
pub trait InstrumentDiscoveryPort: Send + Sync {
    async fn discover(&self) -> Result<Vec<DiscoveredInstrument>, DomainError>;
}
