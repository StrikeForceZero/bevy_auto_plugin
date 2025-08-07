pub mod attribute;
pub mod attribute_args;
pub mod bevy_app_code_gen;
pub mod context;
mod expr_value;
pub mod modes;
mod type_list;
pub mod util;

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
