Automatically registers a type with the app's type registry in module mode.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the type will be registered with these specific generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_register_type]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    struct FooComponent;

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.register_type::<FooComponent>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_register_type(generics(bool))]
    #[auto_register_type(generics(u32))]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    struct FooComponentWithGeneric<T>(T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.register_type::<FooComponentWithGeneric<bool>>();
    //     app.register_type::<FooComponentWithGeneric<u32>>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```