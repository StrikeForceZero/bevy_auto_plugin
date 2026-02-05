Registers a build hook to run custom logic for a type when a plugin builds.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should run this hook.
- `hook = Expr` - Required. Expression that constructs a value implementing `AutoPluginBuildHook<T>` for the target type.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the hook is run for each of these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(T1 = ..., T2 = ...)`.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

struct MyHook;

impl<T: 'static> AutoPluginBuildHook<T> for MyHook {
    fn on_build(&self, _app: &mut App) {
        // custom logic for T
    }
}

#[derive(Component)]
#[auto_plugin_build_hook(plugin = MyPlugin, hook = MyHook)]
struct Foo;
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

struct MyHook;

impl<T: 'static> AutoPluginBuildHook<T> for MyHook {
    fn on_build(&self, _app: &mut App) {}
}

#[derive(Component)]
#[auto_plugin_build_hook(plugin = MyPlugin, hook = MyHook, generics(u32), generics(bool))]
struct Foo<T>(T);
```
