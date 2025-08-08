Automatically registers an event to be added to the app in module mode.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the event will be registered with these specific generic parameters.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::module::prelude::*;

    #[auto_add_event]
    #[derive(Event, Reflect)]
    struct FooEvent;

    /* code gen */
    // pub(super) fn init(app: &mut App) {  
    //     app.add_event::<FooEvent>();
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

    #[auto_add_event(generics(bool))]
    #[derive(Event, Reflect)]
    struct FooEventWithGeneric<T>(T);

    /* code gen */
    // pub(super) fn init(app: &mut App) {
    //     app.add_event::<FooEventWithGeneric<bool>>();
    // }
}

fn plugin(app: &mut App) {
    app.add_plugins(my_plugin::init);
}
```