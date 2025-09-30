Automatically registers an message to be added to the app.

# Parameters

- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the message will be registered with these specific generic parameters.

# Example (without generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_add_message]
#[derive(Message, Reflect)]
struct FooMessage;

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.add_message::<FooMessage>();
}
```

# Example (with generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_add_message(generics(bool))]
#[derive(Message, Reflect)]
struct FooMessageWithGeneric<T>(T);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    /* generated code */
    // app.add_message::<FooMessageWithGeneric<bool>>();
}
```