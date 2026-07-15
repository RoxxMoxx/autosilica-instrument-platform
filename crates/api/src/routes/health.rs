use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "ok"
}
