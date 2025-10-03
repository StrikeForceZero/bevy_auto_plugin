Automatically registers an event to be added to the app in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this event.
- `target([global|entity])` - Optional. (defaults to `global`) Specifies this is a global or entity event: `Event` or `EntityEvent`.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the event will be registered with these specific generic parameters.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Event` or `EntityEvent` (requires `target(global)` or `target(entity)` params respectively). 
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

#[auto_event(plugin = MyPlugin, target(global), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooGlobalEvent(usize);

#[auto_event(plugin = MyPlugin, target(entity), derive(Debug, PartialEq), reflect,  register)]
struct FooEntityEvent(#[event_target] Entity);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;


#[auto_event(plugin = MyPlugin, target(global), generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooGlobalEvent<T>(T);

#[auto_event(plugin = MyPlugin, target(entity), generics(usize), derive(Debug, PartialEq), reflect,  register)]
struct FooEntityEvent<T> {
    #[event_target] 
    entity: Entity,
    value: T,
}
```