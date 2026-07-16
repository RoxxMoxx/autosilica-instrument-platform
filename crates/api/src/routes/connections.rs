use axum::routing::{get, post};
use axum::Router;

use crate::handlers::{connect, disconnect, list_connections};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/connect", post(connect))
        .route("/disconnect", post(disconnect))
        .route("/connections", get(list_connections))
}
