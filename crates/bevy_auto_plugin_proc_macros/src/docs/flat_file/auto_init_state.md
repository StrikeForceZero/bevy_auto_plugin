Automatically initializes a state in the app.

# Example (without generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_init_state]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Foo;

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.init_state::<Foo>();
}
```