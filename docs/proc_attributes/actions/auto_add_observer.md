Automatically adds a global observer

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this observer.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(T1 = ..., T2 = ...)`.

# Notes
- This attribute can be applied to a `use` item; each imported name becomes its own target.
- `use ...::*`, `use ...::self`, and `_` imports are not supported.
- Renames (`as`) are supported and use the local name.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component)]
struct Foo;

#[auto_add_observer(plugin = MyPlugin)]
fn foo_observer(add: On<Add, Foo>, mut commands: Commands) {
    // ...
}
```