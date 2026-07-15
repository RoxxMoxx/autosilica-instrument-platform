use serde::Serialize;

use domain::discovery::DiscoveredInstrument;

/// Wire format for `GET /api/v1/discovery`.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredInstrumentResponse {
    pub id: String,
    pub resource: String,
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
}

impl From<DiscoveredInstrument> for DiscoveredInstrumentResponse {
    fn from(instrument: DiscoveredInstrument) -> Self {
        Self {
            id: instrument.id,
            resource: instrument.resource,
            vendor: instrument.vendor,
            model: instrument.model,
            serial: instrument.serial,
            firmware: instrument.firmware,
        }
    }
}
