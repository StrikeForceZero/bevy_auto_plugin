The main attribute for module mode that processes all auto attributes in the module.

# Parameters
- `init_name=identifier` - Optional. Specifies the name of the generated function that initializes the plugin. (defaults to init)

# Example
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
    pub struct MyComponent;

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.register_type::<MyComponent>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```