Automatically sets `plugin = _` for all `#[auto_*(..)]` macros below it

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin to bind everything below.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_bind_plugin(plugin = MyPlugin)]
#[auto_add_message]
#[auto_register_type]
struct FooEvent(usize);
```
