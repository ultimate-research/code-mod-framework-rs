use std::ffi::{CStr, CString};
use libc::{c_void, c_char};
use std::num::NonZeroU64;

pub mod core;
pub mod ipc;
pub mod dynamic;

pub use self::core::*;
pub use self::ipc::*;
pub use self::dynamic::*;

pub type SwitchResult = libnx_rs::libnx::Result;

#[derive(Debug, Clone)]
pub struct SwitchError {
    pub result: SwitchResult
}

impl std::fmt::Display for SwitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Switch Result error code {}", self.result)
    }
}

impl std::error::Error for SwitchError {
    fn description(&self) -> &str {
        "Switch result error"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl SwitchError {
    pub fn from<T>(result: SwitchResult, val: T) -> Result<T, SwitchError> {
        match result {
            0 => Ok(val),
            _ => Err(SwitchError { result })
        }
    }
}

// Define traits for having a cleaner interface
pub trait IntoVoid {
    fn into(self) -> *const c_void;
}

impl IntoVoid for u64 {
    fn into(self) -> *const c_void {
        self as *const c_void
    }
}

impl IntoVoid for NonZeroU64 {
    fn into(self) -> *const c_void {
        self.get() as *const c_void
    }
}

impl<T> IntoVoid for *const T {
    fn into(self) -> *const c_void {
        self as *const c_void
    }
}

pub trait IntoCString {
    fn into_cstring(&self) -> CString;
    fn as_ptr(&self) -> *const c_char {
        self.into_cstring().as_ptr()
    }
}

impl<T: AsRef<str>> IntoCString for T {
    fn into_cstring(&self) -> CString {
        CString::new(self.as_ref()).unwrap()
    }
}

impl IntoCString for CStr {
    fn into_cstring(&self) -> CString {
        CString::new(self.to_bytes()).unwrap()
    }
}
