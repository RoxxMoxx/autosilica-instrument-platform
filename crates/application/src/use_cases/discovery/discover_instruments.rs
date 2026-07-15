use std::sync::Arc;

use domain::discovery::InstrumentDiscoveryPort;

use crate::dto::DiscoveredInstrumentResponse;
use crate::errors::ApplicationError;

/// Use case: discover instruments currently reachable over VISA
/// (USB, LAN/TCPIP, GPIB, Serial/ASRL) by querying each with `*IDN?`.
pub struct DiscoverInstrumentsUseCase {
    discovery: Arc<dyn InstrumentDiscoveryPort>,
}

impl DiscoverInstrumentsUseCase {
    pub fn new(discovery: Arc<dyn InstrumentDiscoveryPort>) -> Self {
        Self { discovery }
    }

    pub async fn execute(&self) -> Result<Vec<DiscoveredInstrumentResponse>, ApplicationError> {
        let instruments = self.discovery.discover().await?;
        Ok(instruments.into_iter().map(Into::into).collect())
    }
}
