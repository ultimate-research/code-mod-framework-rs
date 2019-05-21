use libc::{c_void, c_char};
use super::{IntoVoid, IntoCString};

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

pub fn get_symbol_addr<T, S>(base: T, name: S) -> u64
    where S: IntoCString,
          T: IntoVoid {
    unsafe {
        S_GetSymbolAddr(
            base.into(),
            name.as_ptr()
        )
    }
}

pub fn find_symbol<S: IntoCString>(name: S) -> u64 {
    unsafe {
        S_FindSymbol(name.as_ptr())
    }
}

pub fn find_symbol_builtin<S: IntoCString>(name: S) -> u64 {
    unsafe {
        S_FindSymbolBuiltin(name.as_ptr())
    }
}

pub fn register_module<T: IntoVoid>(module: T) {
    unsafe {
        S_RegisterModule(module.into())
    }
}

pub fn register_builtin_module<T: IntoVoid>(base: T) {
    unsafe {
        S_RegisterBuiltinModule(base.into())
    }
}

fn dynamic_link_module<V: IntoVoid>(base: V) {
    unsafe {
        S_DynamicLinkModule(base.into())
    }
}

fn replace_module_import<S,V>(base: V, name: S, new_replace: V)
    where V: IntoVoid,
          S: IntoCString,
{
    unsafe {
        S_ReplaceModuleImport(
            base.into(),
            name.as_ptr(),
            new_replace.into()
        )
    }
}

fn replace_import<S, V>(name: S, new_replace: V)
    where V: IntoVoid,
          S: IntoCString,
{
    unsafe {
        S_ReplaceImport(
            name.as_ptr(),
            new_replace.into()
        )
    }
}
