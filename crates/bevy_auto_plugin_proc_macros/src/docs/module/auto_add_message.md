Automatically registers an message to be added to the app in module mode.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the message will be registered with these specific generic parameters.

# Example (without generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[auto_add_message]
    #[derive(Message, Reflect)]
    struct FooMessage;

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.add_message::<FooMessage>();
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

    #[auto_add_message(generics(bool))]
    #[derive(Message, Reflect)]
    struct FooMessageWithGeneric<T>(T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {
    //     app.add_message::<FooMessageWithGeneric<bool>>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```