use libloading::{Library, Symbol};

use super::bindings::*;

/// Candidate VISA shared-library names/paths, in search order, per
/// platform. NI-VISA, Keysight IO Libraries and open equivalents
/// (e.g. `librsvisa`) all expose the same standard VISA C ABI, so any
/// one of these being present is sufficient.
#[cfg(target_os = "windows")]
const CANDIDATE_LIBRARIES: &[&str] = &["visa32.dll", "visa64.dll"];

#[cfg(target_os = "macos")]
const CANDIDATE_LIBRARIES: &[&str] =
    &["/Library/Frameworks/VISA.framework/VISA", "libvisa.dylib"];

#[cfg(all(unix, not(target_os = "macos")))]
const CANDIDATE_LIBRARIES: &[&str] =
    &["libvisa.so", "libvisa.so.7", "librsvisa.so", "libktvisa32.so"];

#[derive(Debug, thiserror::Error)]
pub enum VisaLoadError {
    #[error("no VISA shared library found on this system (tried: {0})")]
    NotFound(String),
    #[error("failed to resolve VISA symbol `{0}`: {1}")]
    MissingSymbol(&'static str, libloading::Error),
}

/// Resolved function pointers for the subset of the VISA C API this
/// crate uses, plus the `Library` handle keeping them valid.
pub struct VisaApi {
    _library: Library,
    pub open_default_rm: ViOpenDefaultRmFn,
    pub find_rsrc: ViFindRsrcFn,
    pub find_next: ViFindNextFn,
    pub open: ViOpenFn,
    pub close: ViCloseFn,
    pub write: ViWriteFn,
    pub read: ViReadFn,
}

// SAFETY: `VisaApi` only ever exposes plain C function pointers and
// keeps the originating `Library` alive; the VISA specification
// requires implementations to be safe to call concurrently from
// multiple threads for the independent session operations used here.
unsafe impl Send for VisaApi {}
unsafe impl Sync for VisaApi {}

impl VisaApi {
    /// Attempts to dynamically load a VISA implementation, trying
    /// each platform-appropriate candidate library name in turn, and
    /// resolving the required symbols from whichever loads first.
    pub fn load() -> Result<Self, VisaLoadError> {
        for candidate in CANDIDATE_LIBRARIES {
            // SAFETY: dynamically loading a system-provided VISA
            // library. We do not execute any of its code until the
            // symbols below are explicitly called.
            if let Ok(library) = unsafe { Library::new(candidate) } {
                return Self::resolve(library);
            }
        }

        Err(VisaLoadError::NotFound(CANDIDATE_LIBRARIES.join(", ")))
    }

    fn resolve(library: Library) -> Result<Self, VisaLoadError> {
        macro_rules! sym {
            ($name:literal, $ty:ty) => {{
                // SAFETY: the resulting function pointer's signature
                // is asserted by us to match the C function exported
                // under this name by any VISA-conformant library.
                let symbol: Symbol<'_, $ty> = unsafe {
                    library
                        .get($name.as_bytes())
                        .map_err(|e| VisaLoadError::MissingSymbol($name, e))?
                };
                *symbol
            }};
        }

        let open_default_rm: ViOpenDefaultRmFn = sym!("viOpenDefaultRM", ViOpenDefaultRmFn);
        let find_rsrc: ViFindRsrcFn = sym!("viFindRsrc", ViFindRsrcFn);
        let find_next: ViFindNextFn = sym!("viFindNext", ViFindNextFn);
        let open: ViOpenFn = sym!("viOpen", ViOpenFn);
        let close: ViCloseFn = sym!("viClose", ViCloseFn);
        let write: ViWriteFn = sym!("viWrite", ViWriteFn);
        let read: ViReadFn = sym!("viRead", ViReadFn);

        Ok(Self {
            _library: library,
            open_default_rm,
            find_rsrc,
            find_next,
            open,
            close,
            write,
            read,
        })
    }
}
