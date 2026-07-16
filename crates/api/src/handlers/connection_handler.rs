use axum::extract::State;
use axum::Json;

use application::dto::{ConnectionRequest, ConnectionResponse};

use crate::error_response::ApiError;
use crate::state::AppState;

pub async fn connect(
    State(state): State<AppState>,
    Json(payload): Json<ConnectionRequest>,
) -> Result<Json<ConnectionResponse>, ApiError> {
    let connection = state.connect_instrument.execute(&payload.resource).await?;
    Ok(Json(connection))
}

pub async fn disconnect(
    State(state): State<AppState>,
    Json(payload): Json<ConnectionRequest>,
) -> Result<Json<ConnectionResponse>, ApiError> {
    let connection = state.disconnect_instrument.execute(&payload.resource).await?;
    Ok(Json(connection))
}

pub async fn list_connections(
    State(state): State<AppState>,
) -> Result<Json<Vec<ConnectionResponse>>, ApiError> {
    let connections = state.list_connections.execute().await?;
    Ok(Json(connections))
}
