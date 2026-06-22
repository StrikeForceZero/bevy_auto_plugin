Automatically inserts a resource with a specific value into the app.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should insert this resource.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `insert(Value)` - Required. Specifies the resource value to insert.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be inserted with these specific generic parameters.
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

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_insert_resource(plugin = MyPlugin, insert(FooResource(42)))]
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
#[auto_insert_resource(plugin = MyPlugin, insert(FooResourceWithGeneric(42)), generics(usize))]
struct FooResourceWithGeneric<T>(T);
```
