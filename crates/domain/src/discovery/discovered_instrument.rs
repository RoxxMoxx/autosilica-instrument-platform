use serde::{Deserialize, Serialize};

/// Instrument metadata obtained via VISA resource discovery and a
/// `*IDN?` query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredInstrument {
    /// Short, human-friendly identifier assigned by the discovery
    /// engine (not necessarily stable across runs).
    pub id: String,
    /// Raw VISA resource string, e.g. `"TCPIP0::192.168.1.100::INSTR"`.
    pub resource: String,
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
}
