Automatically sets `plugin = _` (and optionally `after_build`) for all `#[auto_*(..)]` macros below it

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin to bind everything below.
- `after_build` - Optional. Propagates the `after_build` flag to all `auto_*` macros below so their tokens run at the end of the plugin build.

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
