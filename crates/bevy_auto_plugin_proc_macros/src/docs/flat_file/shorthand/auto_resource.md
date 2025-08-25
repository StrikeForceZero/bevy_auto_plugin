Automatically registers a resource to be added to the app

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be registered with these specific generic parameters.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Resource`. 
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]` 
- `register` - Enables type registration for the `Resource`
  Same as having `#[auto_register_type]`
- `init` - Initializes the `Resource` with default values
  Same as having `#[auto_init_resource]`

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_resource(derive(Debug, Default, PartialEq), reflect,  register)]
struct FooResource(usize);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    //
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_resource(generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooResourceWithGeneric<T>(T);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    //
}
```