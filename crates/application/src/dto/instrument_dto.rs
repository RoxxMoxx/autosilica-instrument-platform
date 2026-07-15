use serde::{Deserialize, Serialize};

use domain::entities::{Instrument, InstrumentStatus};

/// Outbound DTO — decouples the wire format from the domain entity so
/// the two can evolve independently (Interface Segregation /
/// Single Responsibility).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentResponse {
    pub id: String,
    pub name: String,
    pub model: String,
    pub status: String,
}

impl From<Instrument> for InstrumentResponse {
    fn from(instrument: Instrument) -> Self {
        Self {
            id: instrument.id.to_string(),
            name: instrument.name,
            model: instrument.model,
            status: status_to_str(&instrument.status).to_string(),
        }
    }
}

fn status_to_str(status: &InstrumentStatus) -> &'static str {
    match status {
        InstrumentStatus::Registered => "registered",
        InstrumentStatus::Online => "online",
        InstrumentStatus::Offline => "offline",
        InstrumentStatus::Maintenance => "maintenance",
    }
}

/// Inbound DTO for registering a new instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterInstrumentRequest {
    pub name: String,
    pub model: String,
}
