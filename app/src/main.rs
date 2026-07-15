//! Composition root for the AutoSilica Instrument Platform.
//!
//! This binary is intentionally thin: it loads configuration,
//! initializes logging, constructs concrete infrastructure adapters,
//! injects them into application use cases, and hands everything to
//! the `api` layer to build and serve the Axum router.
//!
//! Instrument *communication* (SCPI execution, measurements, etc.) is
//! NOT implemented here. `infrastructure` currently ships an
//! in-memory `InstrumentRepository` and a VISA-backed
//! `InstrumentDiscoveryPort` (resource enumeration + `*IDN?` only); a
//! real driver/adapter layer will be added in a future iteration
//! without requiring changes to `domain`, `application` or `api`.

use std::net::SocketAddr;
use std::sync::Arc;

use application::use_cases::discovery::DiscoverInstrumentsUseCase;
use application::use_cases::instrument::{
    GetInstrumentUseCase, ListInstrumentsUseCase, RegisterInstrumentUseCase,
};
use domain::discovery::InstrumentDiscoveryPort;
use domain::repositories::InstrumentRepository;
use infrastructure::config::Settings;
use infrastructure::logging::init_tracing;
use infrastructure::persistence::InMemoryInstrumentRepository;
use infrastructure::visa::NiVisaDiscoveryBackend;

use api::routes::create_router;
use api::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present; real environment variables always win.
    dotenvy::dotenv().ok();

    let settings = Settings::load()?;
    init_tracing(&settings.logging.level);

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "starting AutoSilica Instrument Platform"
    );

    // -- Dependency wiring (composition root) --------------------------
    let repository: Arc<dyn InstrumentRepository> = Arc::new(InMemoryInstrumentRepository::new());
    let discovery: Arc<dyn InstrumentDiscoveryPort> = Arc::new(NiVisaDiscoveryBackend::new());

    let state = AppState {
        list_instruments: Arc::new(ListInstrumentsUseCase::new(repository.clone())),
        get_instrument: Arc::new(GetInstrumentUseCase::new(repository.clone())),
        register_instrument: Arc::new(RegisterInstrumentUseCase::new(repository.clone())),
        discover_instruments: Arc::new(DiscoverInstrumentsUseCase::new(discovery)),
    };

    let app = create_router(state);

    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    tracing::info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
