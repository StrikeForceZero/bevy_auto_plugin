Automatically registers a resource to be initialized in the app in module mode.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be initialized with these specific generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_init_resource]
    #[derive(Resource, Default, Reflect)]
    #[reflect(Resource)]
    struct FooResource;

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.init_resource::<FooResource>();
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

    #[auto_init_resource(generics(bool))]
    #[derive(Resource, Default, Reflect)]
    #[reflect(Resource)]
    struct FooResourceWithGeneric<T>(T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.init_resource::<FooResourceWithGeneric<bool>>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```