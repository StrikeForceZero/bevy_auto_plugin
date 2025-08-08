Automatically adds a Name component to entities with this component.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the Name component will be added to entities with this component
  using the specified generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[auto_register_type]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_name]
struct FooComponent;

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.register_type::<FooComponent>();
    // app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;

#[auto_register_type(generics(bool))]
#[auto_register_type(generics(u32))]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_name(generics(bool))]
struct FooComponentWithGeneric<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.register_type::<FooComponentWithGeneric<bool>>();
    // app.register_type::<FooComponentWithGeneric<u32>>();
    // app.register_required_components_with::<FooComponentWithGeneric<bool>, Name>(|| Name::new("FooComponentWithGeneric<bool>"));
}
```