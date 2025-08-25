Automatically sets `plugin = _` for all `#[auto_*(..)]` macros below it

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin to bind everything below.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Event, Debug, Default, PartialEq, Reflect)]
#[auto_bind_plugin(plugin = MyPlugin)]
#[auto_add_event]
#[auto_register_type]
struct FooEvent(usize);
```
