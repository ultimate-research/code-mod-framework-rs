use libc::{c_void, c_char};
use std::ffi::CString;

#[link(name="saltysd")]
extern "C" {
    fn S_GetSymbolAddr(base: *const c_void, name: *const c_char) -> u64;
    fn S_FindSymbol(name: *const c_char) -> u64;
    fn S_FindSymbolBuiltin(name: *const c_char) -> u64;
    fn S_RegisterModule(base: *const c_void);
    fn S_RegisterBuiltinModule(base: *const c_void);
    fn S_DynamicLinkModule(base: *const c_void);
    fn S_ReplaceModuleImport(base: *const c_void, name: *const c_char, new_replace: *const c_void);
    fn S_ReplaceImport(name: *const c_char, new_replace: *const c_void);
}

pub fn get_symbol_addr<S: AsRef<str>>(base: *const c_void, name: S) -> u64 {
    unsafe {
        S_GetSymbolAddr(base, CString::new(name.as_ref()).unwrap().as_ptr())
    }
}

pub fn find_symbol<S: AsRef<str>>(name: S) -> u64 {
    unsafe {
        S_FindSymbol(CString::new(name.as_ref()).unwrap().as_ptr())
    }
}

pub fn find_symbol_builtin<S: AsRef<str>>(name: S) -> u64 {
    unsafe {
        S_FindSymbolBuiltin(CString::new(name.as_ref()).unwrap().as_ptr())
    }
}

pub fn register_module(module: *const c_void) {
    unsafe {
        S_RegisterModule(base);
    }
}

