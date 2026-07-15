//! Minimal raw bindings to the subset of the standard VISA C API
//! (as implemented by NI-VISA, Keysight IO Libraries, and other
//! VISA-compliant runtimes) needed for resource discovery and a
//! single `*IDN?` query. Intentionally NOT exhaustive — SCPI
//! execution / measurement APIs are out of scope for this module.
//!
//! Calling convention: on Windows x64 and all Unix-likes, VISA
//! exports use the platform's standard "C" calling convention, so we
//! declare every function pointer as `extern "C"`.

use std::os::raw::c_char;

pub type ViStatus = i32;
pub type ViSession = u32;
pub type ViObject = u32;
pub type ViUInt32 = u32;
pub type ViAccessMode = u32;

/// `VI_SUCCESS`. Any status `>= VI_SUCCESS` is success (0) or an
/// informational completion code (positive); status `< VI_SUCCESS` is
/// an error.
pub const VI_SUCCESS: ViStatus = 0;
/// `VI_NULL`, used here as the default (non-exclusive) access mode
/// passed to `viOpen`.
pub const VI_NULL: ViAccessMode = 0;
/// `VI_ERROR_RSRC_NFOUND` (0xBFFF0011): `viFindRsrc` found no
/// resources matching the search expression. Not a fatal error.
pub const VI_ERROR_RSRC_NFOUND: ViStatus = -1073807343;
/// Standard VISA find-resource description buffer length
/// (`VI_FIND_BUFLEN`).
pub const VI_FIND_BUFLEN: usize = 256;
/// Timeout, in milliseconds, passed to `viOpen`.
pub const VI_OPEN_TIMEOUT_MS: ViUInt32 = 2000;

pub type ViOpenDefaultRmFn = unsafe extern "C" fn(*mut ViSession) -> ViStatus;

pub type ViFindRsrcFn = unsafe extern "C" fn(
    ViSession,
    *const c_char,
    *mut ViObject,
    *mut ViUInt32,
    *mut c_char,
) -> ViStatus;

pub type ViFindNextFn = unsafe extern "C" fn(ViObject, *mut c_char) -> ViStatus;

pub type ViOpenFn = unsafe extern "C" fn(
    ViSession,
    *const c_char,
    ViAccessMode,
    ViUInt32,
    *mut ViSession,
) -> ViStatus;

pub type ViCloseFn = unsafe extern "C" fn(ViObject) -> ViStatus;

pub type ViWriteFn =
    unsafe extern "C" fn(ViSession, *const u8, ViUInt32, *mut ViUInt32) -> ViStatus;

pub type ViReadFn = unsafe extern "C" fn(ViSession, *mut u8, ViUInt32, *mut ViUInt32) -> ViStatus;
