Automatically registers a type with the app's type registry.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the type will be registered with these specific generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_register_type]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct FooComponent;

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.register_type::<FooComponent>();
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_register_type(generics(bool))]
#[auto_register_type(generics(u32))]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct FooComponentWithGeneric<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.register_type::<FooComponentWithGeneric<bool>>();
    // app.register_type::<FooComponentWithGeneric<u32>>();
}
```