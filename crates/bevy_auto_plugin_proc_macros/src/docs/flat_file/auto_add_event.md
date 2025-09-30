Automatically registers an event to be added to the app.

# Parameters

- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the event will be registered with these specific generic parameters.

# Example (without generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_add_event]
#[derive(Message, Reflect)]
struct FooEvent;

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.add_message::<FooEvent>();
}
```

# Example (with generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_add_event(generics(bool))]
#[derive(Message, Reflect)]
struct FooEventWithGeneric<T>(T);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.add_message::<FooEventWithGeneric<bool>>();
}
```