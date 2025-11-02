Automatically inserts a resource with a specific value into the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should insert this resource.
- `resource(Value)` - Required. Specifies the resource value to insert.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be inserted with these specific generic parameters.

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
#[auto_insert_resource(plugin = MyPlugin, init(FooResource(42)))]
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
#[auto_insert_resource(plugin = MyPlugin, init(FooResourceWithGeneric(42)), generics(usize))]
struct FooResourceWithGeneric<T>(T);
```