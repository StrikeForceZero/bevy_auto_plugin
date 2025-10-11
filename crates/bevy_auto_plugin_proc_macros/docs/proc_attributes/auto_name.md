Automatically adds a Name component to entities with this component.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this name.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the Name component will be added to entities with this component
  using the specified generic parameters.
- `name = ...` - Optional. Specified custom name literal to use.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_name(plugin = MyPlugin)]
struct FooComponent;

// This will automatically add a Name component to any entity with FooComponent
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = MyPlugin, generics(bool))]
#[auto_name(plugin = MyPlugin, generics(bool))]
struct FooComponentWithGeneric<T>(T);
```