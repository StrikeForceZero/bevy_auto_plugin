Automatically registers a resource to be added to the app.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should register this resource.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be registered with these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Resource`. 
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]` 
- `register` - Enables type registration for the `Resource`
  Same as having `#[auto_register_type]`
- `init` - Initializes the `Resource` with default values
  Same as having `#[auto_init_resource]`
- `insert(Value)` | `insert(Value1), insert(Value2)` - Inserts the `Resource` with a specific value.

  Same as having:
  ```
  #[auto_insert_resource(insert(Value))]
  ``` 
  or
  ```
  #[auto_insert_resource(insert(Value1))]
  #[auto_insert_resource(insert(Value2))]
  ```

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_resource(plugin = MyPlugin, derive(Debug, Default, PartialEq), reflect,  register)]
struct FooResource(usize);
```

# Example (with init value)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_resource(plugin = MyPlugin, derive(Debug, PartialEq), insert(FooResource(42)))]
struct FooResource(usize);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;


#[auto_resource(plugin = MyPlugin, generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooResourceWithGeneric<T>(T);
```
