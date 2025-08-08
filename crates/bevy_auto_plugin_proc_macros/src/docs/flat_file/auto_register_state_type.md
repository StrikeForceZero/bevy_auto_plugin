Automatically registers `State<T>` and `NextState<T>` types with the app.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_register_state_type]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
struct Foo;

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.register_type::<State<Foo>>();
    // app.register_type::<NextState<Foo>>();
}
```