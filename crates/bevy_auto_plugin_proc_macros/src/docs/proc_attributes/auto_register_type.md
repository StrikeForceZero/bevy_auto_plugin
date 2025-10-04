Automatically registers a type with the app's type registry in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this type.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the type will be registered with these specific generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = MyPlugin)]
struct FooComponent;

// This will register FooComponent with the type registry
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = MyPlugin, generics(bool))]
#[auto_register_type(plugin = MyPlugin, generics(u32))]
struct FooComponentWithGeneric<T>(T);

// This will register FooComponentWithGeneric<bool> and FooComponentWithGeneric<u32>
// with the type registry
```