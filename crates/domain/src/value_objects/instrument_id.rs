use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for an [`crate::entities::Instrument`].
///
/// Wrapping the raw `Uuid` prevents accidentally mixing identifiers
/// belonging to different entity types (a common SOLID / type-safety
/// practice in Clean Architecture domain layers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InstrumentId(pub Uuid);

impl InstrumentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for InstrumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
