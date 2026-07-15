//! Instrument discovery bounded context.
//!
//! Distinct from [`crate::entities::Instrument`]/[`crate::repositories::InstrumentRepository`],
//! which model instruments the platform has explicitly registered.
//! This module models instruments *found* on VISA-addressable buses
//! (USB, LAN/TCPIP, GPIB, Serial/ASRL) via a `*IDN?` query.

mod discovered_instrument;
mod port;

pub use discovered_instrument::DiscoveredInstrument;
pub use port::InstrumentDiscoveryPort;
