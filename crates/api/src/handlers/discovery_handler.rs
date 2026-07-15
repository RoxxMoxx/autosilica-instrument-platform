use axum::extract::State;
use axum::Json;

use application::dto::DiscoveredInstrumentResponse;

use crate::error_response::ApiError;
use crate::state::AppState;

pub async fn discover_instruments(
    State(state): State<AppState>,
) -> Result<Json<Vec<DiscoveredInstrumentResponse>>, ApiError> {
    let instruments = state.discover_instruments.execute().await?;
    Ok(Json(instruments))
}
