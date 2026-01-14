#[cfg(all(feature = "bevy_0_17", feature = "bevy_0_18"))]
compile_error!("features `bevy_0_17` and `bevy_0_18` are mutually exclusive");

#[cfg(not(any(feature = "bevy_0_17", feature = "bevy_0_18")))]
compile_error!("one of the features `bevy_0_17` or `bevy_0_18` must be enabled");

#[cfg(feature = "bevy_0_17")]
#[allow(unused_imports)]
pub use {
    bevy_app_0_17 as bevy_app,
    bevy_ecs_0_17 as bevy_ecs,
    bevy_reflect_0_17 as bevy_reflect,
    bevy_state_0_17 as bevy_state,
};

#[cfg(feature = "bevy_0_18")]
#[allow(unused_imports)]
pub use {
    bevy_app,
    bevy_ecs,
    bevy_reflect,
    bevy_state,
};

#[doc(hidden)]
pub mod __private;

mod codegen;
mod macro_api;
mod syntax;
#[cfg(test)]
mod test_util;
mod util;

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
