Automatically registers an event to be added to the app

# Parameters
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the event will be registered with these specific generic parameters.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Resource`. 
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]` 
- `register` - Enables type registration for the `Resource`
  Same as having `#`[auto_register_type]`

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_event(derive(Debug, Default, PartialEq), reflect,  register)]
struct FooEvent(usize);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    //
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_event(generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooEventWithGeneric<T>(T);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {
    //
}
```