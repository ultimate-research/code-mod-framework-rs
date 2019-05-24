//use libc::{c_void, size_t};
pub type size_t = u64;
//use super::IntoVoid;
//use core::num::NonZeroU64;

#[link(name="saltysd")]
extern "C" {
    fn S_getCodeStart() -> u64;
    fn S_getCodeSize() -> u64;
    pub fn S_findCode(code: *const u8, size: size_t) -> u64;
}

/*
pub fn get_code_start() -> *const c_void {
    unsafe {
        S_getCodeStart() as *const c_void
    }
}

pub fn get_code_size() -> u64 {
    unsafe {
        S_getCodeSize()
    }
}

pub fn find_code(code: &[u8]) -> Option<NonZeroU64> {
    unsafe {
        NonZeroU64::new(S_findCode(code.as_ptr(), code.len() as size_t))
    }
}

pub fn find_code_raw<T: IntoVoid>(code: T, size: size_t) -> Option<NonZeroU64> {
    unsafe {
        NonZeroU64::new(S_findCode(code.into() as *const u8, size))
    }
}*/
