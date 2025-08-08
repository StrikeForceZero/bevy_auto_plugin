Automatically initializes a state in the app in module mode.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::module::prelude::*;

    #[auto_init_state]
    #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
    enum Foo {
        #[default]
        A,
    }

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.init_state::<FooResource>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```