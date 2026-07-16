use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::{Mutex as TokioMutex, RwLock as TokioRwLock};
use tracing::{debug, info, warn};

use domain::connection::{Connection, ConnectionManagerPort, ConnectionState};
use domain::errors::DomainError;

use super::bindings::*;
use super::library::VisaApi;

/// One pooled resource: its domain-level state plus (when open) the
/// raw VISA session handle backing it.
struct PoolEntry {
    connection: Connection,
    /// Open VISA instrument session, present only while a session is
    /// actually held open (state `Connected` or transiently `Busy`).
    session: Option<ViSession>,
}

/// `ConnectionManagerPort` adapter backed by a dynamically-loaded
/// VISA shared library, holding **persistent** per-resource sessions
/// — unlike `discovery_backend`, which opens and immediately closes a
/// session for a single `*IDN?` query, this keeps sessions open and
/// reuses them across calls.
///
/// Thread-safety: all mutable state lives behind `tokio::sync`
/// primitives (`RwLock` for the pool map, `Mutex` for the shared
/// resource-manager session), so the manager is safely shared as
/// `Arc<dyn ConnectionManagerPort>` across HTTP handlers and the
/// background heartbeat task.
pub struct VisaConnectionManager {
    api: OnceLock<Option<Arc<VisaApi>>>,
    /// Persistent VISA default resource-manager session, opened
    /// lazily on first use and reused for every `viOpen` call.
    rm_session: TokioMutex<Option<ViSession>>,
    pool: TokioRwLock<HashMap<String, PoolEntry>>,
    heartbeat_interval: Duration,
}

impl VisaConnectionManager {
    pub fn new(heartbeat_interval: Duration) -> Self {
        Self {
            api: OnceLock::new(),
            rm_session: TokioMutex::new(None),
            pool: TokioRwLock::new(HashMap::new()),
            heartbeat_interval,
        }
    }

    /// Spawns a background Tokio task that periodically runs a
    /// heartbeat pass over the pool. Call once, after wrapping the
    /// manager in an `Arc`.
    pub fn spawn_heartbeat(self: &Arc<Self>) {
        let manager = Arc::clone(self);
        let interval = manager.heartbeat_interval;
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            // The first tick fires immediately; skip it so we don't
            // heartbeat an empty pool the instant the server starts.
            ticker.tick().await;
            loop {
                ticker.tick().await;
                manager.run_heartbeat_once().await;
            }
        });
    }

    fn api(&self) -> Option<Arc<VisaApi>> {
        self.api
            .get_or_init(|| match VisaApi::load() {
                Ok(api) => Some(Arc::new(api)),
                Err(err) => {
                    warn!(
                        error = %err,
                        "no VISA runtime available; connections will fail to open"
                    );
                    None
                }
            })
            .clone()
    }

    async fn resource_manager_session(&self, api: &VisaApi) -> Result<ViSession, DomainError> {
        let mut guard = self.rm_session.lock().await;
        if let Some(session) = *guard {
            return Ok(session);
        }

        let mut session: ViSession = 0;
        // SAFETY: `session` is a valid, uniquely-owned out-pointer.
        let status = unsafe { (api.open_default_rm)(&mut session) };
        if status < VI_SUCCESS {
            return Err(DomainError::Validation(format!(
                "viOpenDefaultRM failed with status {status}"
            )));
        }

        *guard = Some(session);
        Ok(session)
    }

    /// Opens a fresh VISA session to `resource`.
    async fn open_resource(&self, resource: &str) -> Result<ViSession, String> {
        let api = self
            .api()
            .ok_or_else(|| "no VISA runtime available".to_string())?;
        let rm = self
            .resource_manager_session(&api)
            .await
            .map_err(|e| e.to_string())?;

        let resource_c =
            CString::new(resource).map_err(|_| "invalid resource string".to_string())?;
        let mut instrument: ViSession = 0;
        // SAFETY: `resource_c` is valid and NUL-terminated for the
        // duration of this call; `instrument` is a uniquely-owned
        // out-pointer.
        let status = unsafe {
            (api.open)(
                rm,
                resource_c.as_ptr(),
                VI_NULL,
                VI_OPEN_TIMEOUT_MS,
                &mut instrument,
            )
        };
        if status < VI_SUCCESS {
            return Err(format!("viOpen failed with status {status}"));
        }

        Ok(instrument)
    }

    fn close_resource(&self, api: &VisaApi, session: ViSession) {
        // SAFETY: `session` is a valid session handle being
        // relinquished by this call.
        unsafe {
            (api.close)(session);
        }
    }

    /// Sends `*IDN?` on an already-open session and confirms a
    /// response is received, as a lightweight liveness check.
    fn ping(&self, api: &VisaApi, session: ViSession) -> Result<(), String> {
        const QUERY: &[u8] = b"*IDN?\n";

        let mut written: ViUInt32 = 0;
        // SAFETY: `QUERY` is a valid buffer for its given length;
        // `written` is a uniquely-owned out-pointer.
        let status = unsafe {
            (api.write)(
                session,
                QUERY.as_ptr(),
                QUERY.len() as ViUInt32,
                &mut written,
            )
        };
        if status < VI_SUCCESS {
            return Err(format!("viWrite failed with status {status}"));
        }

        let mut buffer = [0u8; 512];
        let mut read: ViUInt32 = 0;
        // SAFETY: `buffer` is a valid, uniquely-owned buffer of the
        // given capacity; `read` is a uniquely-owned out-pointer.
        let status =
            unsafe { (api.read)(session, buffer.as_mut_ptr(), buffer.len() as ViUInt32, &mut read) };
        if status < VI_SUCCESS {
            return Err(format!("viRead failed with status {status}"));
        }

        Ok(())
    }

    /// Runs one heartbeat pass: pings every `Connected` resource
    /// (auto-reconnecting on failure) and retries every
    /// `Disconnected` resource still tracked by the pool.
    async fn run_heartbeat_once(&self) {
        let Some(api) = self.api() else {
            return;
        };

        let (connected, disconnected): (Vec<String>, Vec<String>) = {
            let pool = self.pool.read().await;
            let connected = pool
                .iter()
                .filter(|(_, e)| e.connection.state == ConnectionState::Connected)
                .map(|(resource, _)| resource.clone())
                .collect();
            let disconnected = pool
                .iter()
                .filter(|(_, e)| e.connection.state == ConnectionState::Disconnected)
                .map(|(resource, _)| resource.clone())
                .collect();
            (connected, disconnected)
        };

        for resource in connected {
            self.heartbeat_connected(&api, &resource).await;
        }
        for resource in disconnected {
            self.attempt_reconnect(&api, &resource, "scheduled auto-reconnect")
                .await;
        }
    }

    async fn heartbeat_connected(&self, api: &VisaApi, resource: &str) {
        let session = {
            let mut pool = self.pool.write().await;
            let Some(entry) = pool.get_mut(resource) else {
                return;
            };
            let Some(session) = entry.session else {
                return;
            };
            entry.connection.state = ConnectionState::Busy;
            session
        };

        match self.ping(api, session) {
            Ok(()) => {
                let mut pool = self.pool.write().await;
                if let Some(entry) = pool.get_mut(resource) {
                    entry.connection.state = ConnectionState::Connected;
                    entry.connection.last_heartbeat_at = Some(Utc::now());
                    entry.connection.retry_count = 0;
                    entry.connection.last_error = None;
                }
                debug!(resource, "heartbeat ok");
            }
            Err(err) => {
                warn!(resource, error = %err, "heartbeat failed");
                self.attempt_reconnect(api, resource, "heartbeat failure")
                    .await;
            }
        }
    }

    /// Closes any stale handle for `resource` and attempts to open a
    /// fresh session, updating pool state accordingly.
    async fn attempt_reconnect(&self, api: &VisaApi, resource: &str, reason: &str) {
        {
            let mut pool = self.pool.write().await;
            let Some(entry) = pool.get_mut(resource) else {
                return;
            };
            if let Some(session) = entry.session.take() {
                self.close_resource(api, session);
            }
        }

        match self.open_resource(resource).await {
            Ok(session) => {
                let mut pool = self.pool.write().await;
                if let Some(entry) = pool.get_mut(resource) {
                    entry.session = Some(session);
                    entry.connection.state = ConnectionState::Connected;
                    entry.connection.last_heartbeat_at = Some(Utc::now());
                    entry.connection.retry_count = 0;
                    entry.connection.last_error = None;
                }
                info!(resource, reason, "auto-reconnected");
            }
            Err(err) => {
                let mut pool = self.pool.write().await;
                if let Some(entry) = pool.get_mut(resource) {
                    entry.connection.state = ConnectionState::Disconnected;
                    entry.connection.retry_count += 1;
                    entry.connection.last_error = Some(err.clone());
                }
                warn!(resource, reason, error = %err, "auto-reconnect failed");
            }
        }
    }
}

#[async_trait]
impl ConnectionManagerPort for VisaConnectionManager {
    async fn connect(&self, resource: &str) -> Result<Connection, DomainError> {
        // Fast path: reuse an already-open connection.
        {
            let pool = self.pool.read().await;
            if let Some(entry) = pool.get(resource) {
                if entry.connection.state == ConnectionState::Connected {
                    debug!(resource, "reusing existing connection");
                    return Ok(entry.connection.clone());
                }
            }
        }

        match self.open_resource(resource).await {
            Ok(session) => {
                let now = Utc::now();
                let connection = Connection {
                    resource: resource.to_string(),
                    state: ConnectionState::Connected,
                    connected_at: Some(now),
                    last_heartbeat_at: Some(now),
                    retry_count: 0,
                    last_error: None,
                };

                let mut pool = self.pool.write().await;
                pool.insert(
                    resource.to_string(),
                    PoolEntry {
                        connection: connection.clone(),
                        session: Some(session),
                    },
                );
                info!(resource, "connected");
                Ok(connection)
            }
            Err(err) => {
                let mut pool = self.pool.write().await;
                let previous_retries =
                    pool.get(resource).map(|e| e.connection.retry_count).unwrap_or(0);
                let connection = Connection {
                    resource: resource.to_string(),
                    state: ConnectionState::Error,
                    connected_at: None,
                    last_heartbeat_at: None,
                    retry_count: previous_retries + 1,
                    last_error: Some(err.clone()),
                };
                pool.insert(
                    resource.to_string(),
                    PoolEntry {
                        connection: connection.clone(),
                        session: None,
                    },
                );
                warn!(resource, error = %err, "connect failed");
                Ok(connection)
            }
        }
    }

    async fn disconnect(&self, resource: &str) -> Result<Connection, DomainError> {
        let mut pool = self.pool.write().await;
        let Some(entry) = pool.get_mut(resource) else {
            return Err(DomainError::NotFound(resource.to_string()));
        };

        if let (Some(api), Some(session)) = (self.api(), entry.session.take()) {
            self.close_resource(&api, session);
        }

        entry.connection.state = ConnectionState::Disconnected;
        entry.connection.connected_at = None;
        entry.connection.last_error = None;
        entry.connection.retry_count = 0;

        info!(resource, "disconnected");
        Ok(entry.connection.clone())
    }

    async fn list(&self) -> Result<Vec<Connection>, DomainError> {
        let pool = self.pool.read().await;
        let mut connections: Vec<Connection> =
            pool.values().map(|entry| entry.connection.clone()).collect();
        connections.sort_by(|a, b| a.resource.cmp(&b.resource));
        Ok(connections)
    }
}
