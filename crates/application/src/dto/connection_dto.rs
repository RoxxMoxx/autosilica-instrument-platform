use serde::{Deserialize, Serialize};

use domain::connection::{Connection, ConnectionState};

/// Wire format for connection records returned by `/connect`,
/// `/disconnect` and `/connections`.
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionResponse {
    pub resource: String,
    pub state: String,
    pub connected_at: Option<String>,
    pub last_heartbeat_at: Option<String>,
    pub retry_count: u32,
    pub last_error: Option<String>,
}

impl From<Connection> for ConnectionResponse {
    fn from(connection: Connection) -> Self {
        Self {
            resource: connection.resource,
            state: state_to_str(connection.state).to_string(),
            connected_at: connection.connected_at.map(|t| t.to_rfc3339()),
            last_heartbeat_at: connection.last_heartbeat_at.map(|t| t.to_rfc3339()),
            retry_count: connection.retry_count,
            last_error: connection.last_error,
        }
    }
}

fn state_to_str(state: ConnectionState) -> &'static str {
    match state {
        ConnectionState::Connected => "connected",
        ConnectionState::Disconnected => "disconnected",
        ConnectionState::Busy => "busy",
        ConnectionState::Error => "error",
    }
}

/// Request body for `POST /api/v1/connect` and `POST /api/v1/disconnect`.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionRequest {
    pub resource: String,
}
