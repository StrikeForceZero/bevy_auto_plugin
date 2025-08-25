Automatically adds a global observer

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[derive(Component)]
struct Foo;

#[auto_observer]
fn foo_observer(trigger: Trigger<OnAdd, Foo>, mut commands: Commands) {
    // ...
}

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    //
}
```