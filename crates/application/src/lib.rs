//! Application layer.
//!
//! Hosts use cases (a.k.a. interactors) that orchestrate domain
//! entities and repository ports to fulfil application-specific
//! business rules. This layer depends on `domain` only — it knows
//! nothing about HTTP, databases, or configuration.

pub mod dto;
pub mod errors;
pub mod use_cases;
