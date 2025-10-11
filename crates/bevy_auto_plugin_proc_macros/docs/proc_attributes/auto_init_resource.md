Automatically registers a resource to be initialized in the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should initialize this resource.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be initialized with these specific generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_init_resource(plugin = MyPlugin)]
struct FooResource(usize);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MyPlugin, generics(usize))]
#[auto_init_resource(plugin = MyPlugin, generics(usize))]
struct FooResourceWithGeneric<T>(T);
```