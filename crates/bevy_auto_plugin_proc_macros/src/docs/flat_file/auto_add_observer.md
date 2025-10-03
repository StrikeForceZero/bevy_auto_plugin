Automatically adds an observer to the Bevy `App`.

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[derive(Component)]
struct Foo;

#[auto_add_observer]
fn foo_observer(add: On<Add, Foo>, mut commands: Commands) {
    // ...
}

#[auto_plugin]
fn setup(app: &mut App) {
    // ...
}
```