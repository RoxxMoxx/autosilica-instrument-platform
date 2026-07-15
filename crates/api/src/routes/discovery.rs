use axum::routing::get;
use axum::Router;

use crate::handlers::discover_instruments;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/discovery", get(discover_instruments))
}
