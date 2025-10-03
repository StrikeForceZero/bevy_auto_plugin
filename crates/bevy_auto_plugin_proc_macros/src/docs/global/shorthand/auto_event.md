### NOTE: has not been updated to bevy 0.17 yet - See: https://github.com/StrikeForceZero/bevy_auto_plugin/issues/23

Automatically registers an event to be added to the app in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this event.
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
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_event(plugin = MyPlugin, derive(Debug, Default, PartialEq), reflect,  register)]
struct FooEvent(usize);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;


#[auto_event(plugin = MyPlugin, generics(usize), derive(Debug, Default, PartialEq), reflect,  register)]
struct FooEventWithGeneric<T>(T);
```