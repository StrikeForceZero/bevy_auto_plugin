#[doc(hidden)]
pub mod __private;

mod codegen;
mod macro_api;
mod syntax;
#[cfg(test)]
mod test_util;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn __wasm_call_ctors();
}

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}
