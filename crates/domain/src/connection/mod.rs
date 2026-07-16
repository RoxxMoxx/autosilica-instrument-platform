//! Instrument connection-pool bounded context.
//!
//! Models persistent, reusable connections to VISA resources and the
//! states they move through (`Connected`, `Disconnected`, `Busy`,
//! `Error`). Distinct from `discovery` (which only enumerates and
//! `*IDN?`-queries resources) and from the registered-`Instrument`
//! context.

mod connection;
mod port;

pub use connection::{Connection, ConnectionState};
pub use port::ConnectionManagerPort;
