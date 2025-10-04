Automatically adds a global observer

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this observer.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

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