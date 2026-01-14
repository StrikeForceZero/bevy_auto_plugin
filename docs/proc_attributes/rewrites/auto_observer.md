Automatically adds a global observer

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this observer.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component)]
struct Foo;

#[auto_observer(plugin = MyPlugin)]
fn foo_observer(add: On<Add, Foo>, mut commands: Commands) {
    // ...
}
```