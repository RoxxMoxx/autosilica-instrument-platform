use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::value_objects::InstrumentId;

/// Lifecycle status of an instrument record.
///
/// NOTE: this models the *registration* lifecycle only. Actual
/// instrument communication/connectivity is intentionally out of
/// scope for this skeleton and will live in a future infrastructure
/// adapter (e.g. `crates/drivers`) implementing domain ports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstrumentStatus {
    Registered,
    Online,
    Offline,
    Maintenance,
}

/// Core entity representing a laboratory/industrial instrument known
/// to the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub id: InstrumentId,
    pub name: String,
    pub model: String,
    pub status: InstrumentStatus,
    pub created_at: DateTime<Utc>,
}

impl Instrument {
    /// Creates a new instrument in the `Registered` state.
    pub fn new(name: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            id: InstrumentId::new(),
            name: name.into(),
            model: model.into(),
            status: InstrumentStatus::Registered,
            created_at: Utc::now(),
        }
    }
}
