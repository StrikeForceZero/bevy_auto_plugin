Automatically sets `plugin = _` (and optionally `post_build`) for all `#[auto_*(..)]` macros below it

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin to bind everything below.
- `post_build` - Optional. Propagates the `post_build` flag to all `auto_*` macros below so their tokens run at the end of the plugin build.

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
