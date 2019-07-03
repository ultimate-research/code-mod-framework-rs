use libc::{c_void, size_t};
use super::*;
use nx::sys::Handle;

extern "C" {
    fn S_Init();
    fn S_Deinit() -> SwitchResult;
    fn S_Term() -> SwitchResult;
    fn S_Restore() -> SwitchResult;
    fn S_Memcpy(to: *const c_void, from: *const c_void, size: size_t) -> SwitchResult;
    fn S_GetSDCard(retrieve: *mut Handle) -> SwitchResult;
}

pub fn init() {
    unsafe { S_Init() }
}

pub fn deinit() -> Result<(), SwitchError> {
    unsafe {
        SwitchError::from(S_Deinit(), ())
    }
}

pub fn term() -> Result<(), SwitchError> {
    unsafe {
        SwitchError::from(S_Term(), ())
    }
}

pub fn restore() -> Result<(), SwitchError> {
    unsafe {
        SwitchError::from(S_Restore(), ())
    }
}

pub fn memcpy<V: IntoVoid>(to: V, from: &[u8]) -> Result<(), SwitchError> {
    unsafe {
        SwitchError::from(
            S_Memcpy(
                to.into(),
                from.as_ptr() as *const c_void,
                from.len() as size_t
            ),
            ()
        )
    }
}

pub fn memcpy_raw<V: IntoVoid, S: Into<size_t>>(to: V, from: V, size: S) -> Result<(), SwitchError> {
    unsafe {
        SwitchError::from(
            S_Memcpy(
                to.into(),
                from.into(),
                size.into()
            ),
            ()
        )
    }
}

pub fn get_sd_card() -> Result<Handle, SwitchError> {
    let mut handle: Handle = 0;
    unsafe {
        SwitchError::from(
            S_GetSDCard(
                (&mut handle) as *mut Handle
            ),
            handle
        )
    }
}
