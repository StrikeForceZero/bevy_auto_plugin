Automatically registers `State<T>` and `NextState<T>` types with the app in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register these state types.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = MyPlugin)]
#[auto_register_state_type(plugin = MyPlugin)]
enum FooState {
    #[default]
    Start,
    End,
}

// This will register both State<FooState> and NextState<FooState> with the type registry
```