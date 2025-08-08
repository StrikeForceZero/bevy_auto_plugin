Automatically adds a Name component to entities with this component in module mode.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the Name component will be added to entities with this component
  using the specified generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::module::prelude::*;

    #[auto_register_type]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    #[auto_name]
    struct FooComponent;

    /* code gen */
    // pub(super) fn init(app: &mut App) {
    //     app.register_type::<FooComponent>();
    //     app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::module::prelude::*;

    #[auto_register_type(generics(bool))]
    #[auto_register_type(generics(u32))]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    #[auto_name(generics(bool))]
    struct FooComponentWithGeneric<T>(T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.register_type::<FooComponentWithGeneric<bool>>();
    //     app.register_type::<FooComponentWithGeneric<u32>>();
    //     app.register_required_components_with::<FooComponentWithGeneric<boo>, Name>(|| Name::new("FooComponentWithGeneric<boo>"));
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```