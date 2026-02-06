Automatically initializes a sub state in the app.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should initialize this sub state.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.

# Notes
- This attribute can be applied to a `use` item; each imported name becomes its own target.
- `use ...::*`, `use ...::self`, and `_` imports are not supported.
- Renames (`as`) are supported and use the local name.

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