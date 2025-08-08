Automatically inserts a resource with a specific value into the app.

# Parameters
- `resource(Value)` - Required. Specifies the resource value to insert.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_insert_resource(resource(FooResource(42)))]
struct FooResource(usize);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.insert_resource(FooResource(42));
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_insert_resource(resource(FooResourceWithGeneric(42)), generics(usize))]
struct FooResourceWithGeneric<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.insert_resource(FooResourceWithGeneric::<usize>(42));
}
```