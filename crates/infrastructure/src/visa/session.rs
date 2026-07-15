use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use thiserror::Error;

use super::bindings::*;
use super::library::VisaApi;

#[derive(Debug, Error)]
pub enum VisaSessionError {
    #[error("VISA call `{0}` failed with status {1}")]
    Call(&'static str, ViStatus),
    #[error("resource string contained an interior NUL byte")]
    InvalidResourceString,
}

/// Safe(r) wrapper around a VISA default resource-manager session.
///
/// Closes the resource manager session automatically on drop.
pub struct VisaResourceManager<'a> {
    api: &'a VisaApi,
    session: ViSession,
}

impl<'a> VisaResourceManager<'a> {
    /// Opens the VISA default resource manager (`viOpenDefaultRM`).
    pub fn open(api: &'a VisaApi) -> Result<Self, VisaSessionError> {
        let mut session: ViSession = 0;
        // SAFETY: `session` is a valid, uniquely-owned out-pointer for
        // the duration of this call.
        let status = unsafe { (api.open_default_rm)(&mut session) };
        if status < VI_SUCCESS {
            return Err(VisaSessionError::Call("viOpenDefaultRM", status));
        }
        Ok(Self { api, session })
    }

    /// Finds every resource matching a VISA search expression, e.g.
    /// `"USB?*INSTR"`, `"TCPIP?*INSTR"`, `"GPIB?*INSTR"`, `"ASRL?*INSTR"`.
    pub fn find_resources(&self, expr: &str) -> Result<Vec<String>, VisaSessionError> {
        let expr = CString::new(expr).map_err(|_| VisaSessionError::InvalidResourceString)?;
        let mut find_list: ViObject = 0;
        let mut count: ViUInt32 = 0;
        let mut desc = [0 as c_char; VI_FIND_BUFLEN];

        // SAFETY: all pointers are valid, uniquely-owned, and sized
        // per the VISA specification for the duration of this call.
        let status = unsafe {
            (self.api.find_rsrc)(
                self.session,
                expr.as_ptr(),
                &mut find_list,
                &mut count,
                desc.as_mut_ptr(),
            )
        };

        if status == VI_ERROR_RSRC_NFOUND {
            // No matches for this interface class — not a failure.
            return Ok(Vec::new());
        }
        if status < VI_SUCCESS {
            return Err(VisaSessionError::Call("viFindRsrc", status));
        }

        let mut resources = Vec::with_capacity(count as usize);
        resources.push(desc_to_string(&desc));

        for _ in 1..count {
            let mut next_desc = [0 as c_char; VI_FIND_BUFLEN];
            // SAFETY: `find_list` is the valid list handle returned
            // above; `next_desc` is a uniquely-owned out-buffer.
            let status = unsafe { (self.api.find_next)(find_list, next_desc.as_mut_ptr()) };
            if status < VI_SUCCESS {
                break;
            }
            resources.push(desc_to_string(&next_desc));
        }

        // SAFETY: `find_list` is a valid VISA object handle owned by
        // this call; VISA find lists are closed via `viClose`.
        unsafe {
            (self.api.close)(find_list);
        }

        Ok(resources)
    }

    /// Opens `resource`, sends `*IDN?`, and returns the raw response
    /// string (parsing into vendor/model/serial/firmware happens
    /// separately, see [`super::idn::parse_idn`]).
    pub fn query_idn(&self, resource: &str) -> Result<String, VisaSessionError> {
        let resource_c =
            CString::new(resource).map_err(|_| VisaSessionError::InvalidResourceString)?;

        let mut instrument: ViSession = 0;
        // SAFETY: `resource_c` is a valid, NUL-terminated C string for
        // the duration of this call; `instrument` is a uniquely-owned
        // out-pointer.
        let status = unsafe {
            (self.api.open)(
                self.session,
                resource_c.as_ptr(),
                VI_NULL,
                VI_OPEN_TIMEOUT_MS,
                &mut instrument,
            )
        };
        if status < VI_SUCCESS {
            return Err(VisaSessionError::Call("viOpen", status));
        }

        let result = self.write_and_read_idn(instrument);

        // SAFETY: `instrument` is a valid, uniquely-owned session
        // handle opened above.
        unsafe {
            (self.api.close)(instrument);
        }

        result
    }

    fn write_and_read_idn(&self, instrument: ViSession) -> Result<String, VisaSessionError> {
        const QUERY: &[u8] = b"*IDN?\n";

        let mut written: ViUInt32 = 0;
        // SAFETY: `QUERY` is a valid buffer of the given length;
        // `written` is a uniquely-owned out-pointer.
        let status = unsafe {
            (self.api.write)(
                instrument,
                QUERY.as_ptr(),
                QUERY.len() as ViUInt32,
                &mut written,
            )
        };
        if status < VI_SUCCESS {
            return Err(VisaSessionError::Call("viWrite", status));
        }

        let mut buffer = [0u8; 512];
        let mut read: ViUInt32 = 0;
        // SAFETY: `buffer` is a valid, uniquely-owned buffer of the
        // given capacity; `read` is a uniquely-owned out-pointer.
        let status = unsafe {
            (self.api.read)(
                instrument,
                buffer.as_mut_ptr(),
                buffer.len() as ViUInt32,
                &mut read,
            )
        };
        if status < VI_SUCCESS {
            return Err(VisaSessionError::Call("viRead", status));
        }

        Ok(String::from_utf8_lossy(&buffer[..read as usize])
            .trim()
            .to_string())
    }
}

impl<'a> Drop for VisaResourceManager<'a> {
    fn drop(&mut self) {
        // SAFETY: `self.session` is a valid session handle owned by
        // this struct for its entire lifetime.
        unsafe {
            (self.api.close)(self.session);
        }
    }
}

fn desc_to_string(desc: &[c_char; VI_FIND_BUFLEN]) -> String {
    // SAFETY: `desc` was populated by `viFindRsrc`/`viFindNext`, which
    // per the VISA specification NUL-terminate the description
    // within the provided `VI_FIND_BUFLEN`-sized buffer.
    unsafe { CStr::from_ptr(desc.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}
