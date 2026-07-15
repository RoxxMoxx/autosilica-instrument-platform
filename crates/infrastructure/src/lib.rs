//! Infrastructure layer.
//!
//! Provides concrete implementations ("adapters") for the ports
//! defined in `domain` (e.g. `InstrumentRepository`), plus
//! cross-cutting technical concerns: configuration loading and
//! logging/tracing setup. Depends on `domain` only — never on
//! `application` or `api` — keeping the dependency arrows pointing
//! inward, per Clean Architecture / Dependency Inversion.
//!
//! NOTE: instrument communication drivers are intentionally not part
//! of this skeleton. A future `drivers` (or similar) crate would live
//! alongside `persistence` here and implement dedicated domain ports.

pub mod config;
pub mod errors;
pub mod logging;
pub mod persistence;
pub mod visa;
