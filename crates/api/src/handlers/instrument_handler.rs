use axum::extract::{Path, State};
use axum::Json;

use application::dto::{InstrumentResponse, RegisterInstrumentRequest};

use crate::error_response::ApiError;
use crate::state::AppState;

pub async fn list_instruments(
    State(state): State<AppState>,
) -> Result<Json<Vec<InstrumentResponse>>, ApiError> {
    let instruments = state.list_instruments.execute().await?;
    Ok(Json(instruments))
}

pub async fn get_instrument(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<InstrumentResponse>, ApiError> {
    let instrument = state.get_instrument.execute(&id).await?;
    Ok(Json(instrument))
}

pub async fn register_instrument(
    State(state): State<AppState>,
    Json(payload): Json<RegisterInstrumentRequest>,
) -> Result<Json<InstrumentResponse>, ApiError> {
    let instrument = state.register_instrument.execute(payload).await?;
    Ok(Json(instrument))
}
