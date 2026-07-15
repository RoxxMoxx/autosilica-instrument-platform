use axum::routing::get;
use axum::Router;

use crate::handlers::{get_instrument, list_instruments, register_instrument};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/instruments", get(list_instruments).post(register_instrument))
        .route("/instruments/:id", get(get_instrument))
}
