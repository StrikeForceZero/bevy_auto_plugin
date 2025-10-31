Automatically initializes a sub state in the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should initialize this sub state.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = MyPlugin)]
#[auto_register_state_type(plugin = MyPlugin)]
enum AppState {
    #[default]
    Menu,
    InGame
}

#[derive(SubStates, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[source(AppState = AppState::InGame)]
#[auto_init_sub_state(plugin = MyPlugin)]
#[auto_register_state_type(plugin = MyPlugin)]
enum GamePhase {
    #[default]
    Setup,
    Battle,
    Conclusion
}
```