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

/// Hook invoked during a plugin's build for a specific target type `T`.
///
/// Register a hook by annotating the target type with [`bevy_auto_plugin::prelude::auto_plugin_build_hook`].
///
/// The `hook` expression is evaluated when the plugin builds. Use `after_build` on the
/// attribute to run the hook after the build body instead of before.
///
/// # Example
/// ```rust
/// use bevy_app::prelude::*;
/// use bevy_ecs::prelude::*;
/// use bevy_auto_plugin::prelude::*;
///
/// #[derive(AutoPlugin)]
/// #[auto_plugin(impl_plugin_trait)]
/// struct MyPlugin;
///
/// struct SpawnComponentHook;
///
/// impl<T: 'static> AutoPluginBuildHook<T> for SpawnComponentHook where T: Component + Default {
///     fn on_build(&self, app: &mut App) {
///         app.world_mut().spawn(T::default());
///     }
/// }
///
/// #[auto_component(plugin = MyPlugin, derive(Default))]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = SpawnComponentHook)]
/// struct MyComponent;
/// ```
/// Generates the equivalent to:
/// ```rust
///  use bevy_app::prelude::*;
/// use bevy_ecs::prelude::*;
///
/// #[derive(Component, Default)]
/// struct MyComponent;
///
/// struct MyPlugin;
/// impl Plugin for MyPlugin {
///    fn build(&self, app: &mut App) {
///        app.world_mut().spawn(MyComponent);
///    }
/// }
/// ```
pub trait AutoPluginBuildHook<T: 'static> {
    /// Called during plugin build for the target type `T`.
    fn on_build(&self, app: &mut bevy_app::App);
}
