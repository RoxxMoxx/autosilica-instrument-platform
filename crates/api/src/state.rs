use std::sync::Arc;

use application::use_cases::discovery::DiscoverInstrumentsUseCase;
use application::use_cases::instrument::{
    GetInstrumentUseCase, ListInstrumentsUseCase, RegisterInstrumentUseCase,
};

/// Shared application state injected into Axum handlers.
///
/// Holds `Arc`-wrapped use cases rather than raw repositories, so
/// handlers stay thin and only ever talk to the application layer
/// (Interface Segregation / Dependency Inversion).
#[derive(Clone)]
pub struct AppState {
    pub list_instruments: Arc<ListInstrumentsUseCase>,
    pub get_instrument: Arc<GetInstrumentUseCase>,
    pub register_instrument: Arc<RegisterInstrumentUseCase>,
    pub discover_instruments: Arc<DiscoverInstrumentsUseCase>,
}
