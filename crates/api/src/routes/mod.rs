mod discovery;
mod health;
mod instruments;

use axum::Router;
use tower_http::cors::CorsLayer;

use crate::middleware::request_id_layer;
use crate::state::AppState;

/// Builds the top-level Axum router, mounting all route groups and
/// shared middleware.
pub fn create_router(state: AppState) -> Router {
    let api_v1 = instruments::routes().merge(discovery::routes());

    Router::new()
        .merge(health::routes())
        .nest("/api/v1", api_v1)
        .layer(request_id_layer())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
