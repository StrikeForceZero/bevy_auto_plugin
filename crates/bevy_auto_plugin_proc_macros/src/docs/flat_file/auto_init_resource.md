Automatically registers a resource to be initialized in the app.

# Parameters

- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be initialized with these specific generic parameters.

# Example (without generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[auto_init_resource]
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct FooResource;

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.init_resource::<FooResource>();
}
```

# Example (with generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[auto_init_resource(generics(bool))]
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct FooResourceWithGeneric<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.init_resource::<FooResourceWithGeneric<bool>>();
}
```