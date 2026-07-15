use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// HTTP server settings.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

/// Logging/tracing settings.
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
}

/// Root application configuration.
///
/// Load order (later sources override earlier ones):
///   1. `config/default.toml`
///   2. `config/{APP_ENV}.toml` (optional, e.g. `development`, `production`)
///   3. Environment variables prefixed `AUTOSILICA__`, double-underscore
///      separated (e.g. `AUTOSILICA__SERVER__PORT=9090`)
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub logging: LoggingSettings,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{run_mode}")).required(false))
            .add_source(
                Environment::with_prefix("AUTOSILICA")
                    .separator("__")
                    .try_parsing(true),
            );

        builder.build()?.try_deserialize()
    }
}
