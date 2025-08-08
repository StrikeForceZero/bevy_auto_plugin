Automatically registers `State<T>` and `NextState<T>` types with the app in module mode.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_register_state_type]
    #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
    enum Foo {
        #[default]
        A,
    }

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.register_type::<State<Foo>>();
    //     app.register_type::<NextState<Foo>>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```