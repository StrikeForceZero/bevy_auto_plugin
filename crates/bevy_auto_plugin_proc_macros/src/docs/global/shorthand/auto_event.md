Automatically registers an event to be added to the app in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this event.
- `global` - Optional. Specifies this is a global event: `Event`.
- `entity` | `entity(<entity_event args>)` - Optional. Specifies this is an event emitting from an `Entity`: `EntityEvent`.
  - `entity_event` args:
    - `propagate` | `propagate = T` | `propagate(T)`: Optional. (defaults to `ChildOf`)
    - `auto_propagate`: Optional.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the event will be registered with these specific generic parameters.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Event` or `EntityEvent` (requires `global` or `entity` flags respectively). 
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]` 
- `register` - Enables type registration for the `Resource`
  Same as having `#`[auto_register_type]`

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_event(plugin = MyPlugin, global, derive(Debug, Default, PartialEq), reflect,  register)]
struct FooGlobalEvent(usize);

#[auto_event(plugin = MyPlugin, entity(propagate, auto_propagate), derive(Debug, PartialEq), reflect,  register)]
struct FooEntityEvent(#[event_target] Entity);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;


#[auto_event(plugin = MyPlugin, global, generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooGlobalEvent<T>(T);

#[auto_event(plugin = MyPlugin, entity(propagate, auto_propagate), generics(usize), derive(Debug, PartialEq), reflect,  register)]
struct FooEntityEvent<T> {
    #[event_target] 
    entity: Entity,
    value: T,
}
```