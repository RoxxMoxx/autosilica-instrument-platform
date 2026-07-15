//! VISA-based instrument discovery adapter.
//!
//! Implements `domain::discovery::InstrumentDiscoveryPort` on top of
//! a dynamically-loaded VISA shared library (NI-VISA, Keysight IO
//! Libraries, or any other implementation exposing the standard VISA
//! C ABI). The library is loaded at *runtime* via `libloading`
//! (dlopen/LoadLibrary), NOT linked at compile time — this crate
//! builds and the platform runs fine even on machines without any
//! VISA implementation installed; discovery then simply returns an
//! empty list.
//!
//! Submodules:
//! - `bindings`   — raw VISA C ABI types, constants, function-pointer signatures.
//! - `library`    — locates and dynamically loads a VISA shared library.
//! - `session`    — safe wrapper around a VISA resource-manager session.
//! - `idn`        — parses `*IDN?` responses into vendor/model/serial/firmware.
//! - `discovery_backend` — the `InstrumentDiscoveryPort` adapter itself.

mod bindings;
mod discovery_backend;
mod idn;
mod library;
mod session;

pub use discovery_backend::NiVisaDiscoveryBackend;
