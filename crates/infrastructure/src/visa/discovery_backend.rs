use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use tracing::{debug, warn};

use domain::discovery::{DiscoveredInstrument, InstrumentDiscoveryPort};
use domain::errors::DomainError;

use super::idn::parse_idn;
use super::library::VisaApi;
use super::session::VisaResourceManager;

/// VISA search expressions for each supported interface class.
const RESOURCE_PATTERNS: &[(&str, &str)] = &[
    ("USB", "USB?*INSTR"),
    ("LAN", "TCPIP?*INSTR"),
    ("GPIB", "GPIB?*INSTR"),
    ("Serial", "ASRL?*INSTR"),
];

/// `InstrumentDiscoveryPort` adapter backed by a dynamically-loaded
/// VISA shared library (NI-VISA or compatible).
///
/// The library is probed lazily on first use and the result (loaded
/// or unavailable) is cached for the lifetime of this backend. If no
/// VISA runtime is installed on the host, a warning is logged once
/// and every subsequent `discover()` call simply returns an empty
/// list rather than failing the endpoint — discovery availability is
/// inherently host/hardware dependent.
pub struct NiVisaDiscoveryBackend {
    api: OnceLock<Option<Arc<VisaApi>>>,
}

impl NiVisaDiscoveryBackend {
    pub fn new() -> Self {
        Self {
            api: OnceLock::new(),
        }
    }

    fn api(&self) -> Option<Arc<VisaApi>> {
        self.api
            .get_or_init(|| match VisaApi::load() {
                Ok(api) => Some(Arc::new(api)),
                Err(err) => {
                    warn!(
                        error = %err,
                        "no VISA runtime available; instrument discovery will return no results"
                    );
                    None
                }
            })
            .clone()
    }
}

impl Default for NiVisaDiscoveryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InstrumentDiscoveryPort for NiVisaDiscoveryBackend {
    async fn discover(&self) -> Result<Vec<DiscoveredInstrument>, DomainError> {
        let Some(api) = self.api() else {
            return Ok(Vec::new());
        };

        // VISA calls are blocking C FFI calls; keep them off the
        // async executor's worker threads.
        let instruments = tokio::task::spawn_blocking(move || discover_blocking(&api))
            .await
            .map_err(|err| DomainError::Validation(format!("discovery task panicked: {err}")))?;

        Ok(instruments)
    }
}

fn discover_blocking(api: &VisaApi) -> Vec<DiscoveredInstrument> {
    let rm = match VisaResourceManager::open(api) {
        Ok(rm) => rm,
        Err(err) => {
            warn!(error = %err, "failed to open VISA default resource manager");
            return Vec::new();
        }
    };

    let mut resources: Vec<String> = Vec::new();
    for (label, pattern) in RESOURCE_PATTERNS {
        match rm.find_resources(pattern) {
            Ok(found) => {
                debug!(interface = %label, count = found.len(), "VISA find_resources completed");
                resources.extend(found);
            }
            Err(err) => {
                warn!(interface = %label, error = %err, "VISA find_resources failed for interface class");
            }
        }
    }

    resources.sort();
    resources.dedup();

    let mut discovered = Vec::with_capacity(resources.len());
    for (index, resource) in resources.into_iter().enumerate() {
        match rm.query_idn(&resource) {
            Ok(raw_idn) => {
                let fields = parse_idn(&raw_idn);
                discovered.push(DiscoveredInstrument {
                    id: generate_id(index, &fields.model),
                    resource,
                    vendor: fields.vendor,
                    model: fields.model,
                    serial: fields.serial,
                    firmware: fields.firmware,
                });
            }
            Err(err) => {
                warn!(resource = %resource, error = %err, "*IDN? query failed; skipping resource");
            }
        }
    }

    discovered
}

/// Generates a short, human-friendly identifier for a discovered
/// instrument from its model string plus a stable index.
///
/// This is a simple slug+index scheme; a future enhancement could map
/// vendor/model to a richer, domain-specific alias (e.g. `"scope01"`)
/// via a configurable lookup table.
fn generate_id(index: usize, model: &str) -> String {
    let slug: String = model
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let slug = slug.trim_matches('-');

    if slug.is_empty() {
        format!("instrument-{:02}", index + 1)
    } else {
        format!("{slug}-{:02}", index + 1)
    }
}
