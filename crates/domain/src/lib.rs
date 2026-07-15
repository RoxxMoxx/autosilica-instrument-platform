//! Domain layer.
//!
//! This crate contains the enterprise-wide business rules of the
//! AutoSilica Instrument Platform: entities, value objects and the
//! repository "ports" (traits) that outer layers must implement.
//!
//! Following Clean Architecture / SOLID, this crate has ZERO knowledge
//! of persistence, HTTP, configuration or instrument communication.
//! It must never depend on `application`, `infrastructure` or `api`.

pub mod discovery;
pub mod entities;
pub mod errors;
pub mod repositories;
pub mod value_objects;
