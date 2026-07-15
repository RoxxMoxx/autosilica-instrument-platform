//! API (interface) layer.
//!
//! Wires HTTP concerns (Axum routes/handlers, request/response
//! mapping, middleware) on top of the `application` layer's use
//! cases. This is the outermost layer in the Clean Architecture
//! sense — it depends inward on `application` and `domain`, and is
//! itself depended on by nothing (the `app` binary composes it).

pub mod error_response;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod state;
