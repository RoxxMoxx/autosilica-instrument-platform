use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Lifecycle state of a pooled connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    /// A live, idle session is open and ready for use.
    Connected,
    /// No session is currently open for this resource.
    Disconnected,
    /// A session is open but momentarily occupied (e.g. a heartbeat
    /// or command is in flight).
    Busy,
    /// The most recent connect/reconnect attempt failed.
    Error,
}

/// Point-in-time record of a pooled connection to a VISA resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub resource: String,
    pub state: ConnectionState,
    pub connected_at: Option<DateTime<Utc>>,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub last_error: Option<String>,
}

impl Connection {
    /// A fresh, never-connected record for `resource`.
    pub fn disconnected(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            state: ConnectionState::Disconnected,
            connected_at: None,
            last_heartbeat_at: None,
            retry_count: 0,
            last_error: None,
        }
    }
}
