Automatically registers a component to be added to the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this component.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the component will be registered with these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Component`. 
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]` 
- `register` - Enables type registration for the `Component`
  Same as having `#[auto_register_type]`
- `auto_name | auto_name = ...` - Enables adding a required component of `Name` with the `Component`'s concrete name or custom name literal if specified.
  Same as having `#[auto_name]` or `#[auto_name = ...]`

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_component(plugin = MyPlugin, derive(Debug, Default, PartialEq), reflect,  register, auto_name)]
struct FooComponent(usize);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;


#[auto_component(plugin = MyPlugin, generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooComponentWithGeneric<T>(T);
```