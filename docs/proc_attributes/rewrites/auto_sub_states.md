Automatically initializes a sub-state in the app.

# Parameters

Parameters marked **repeatable** may appear more than once within the same attribute.
All other parameters may appear at most once. Generic parameters are not supported.

- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should initialize this state.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `SubStates`.
  Passes through any additional derives listed.
  When enabled, `SubStates` include these additional derives:  
    - `SubStates`
    - `Debug`
    - `Default`
    - `Clone`
    - `PartialEq`
    - `Eq`
    - `Hash`
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]`
- `register` - Enables type registration for the `SubStates`
  Same as having `#[auto_register_type]` and `#[auto_register_state_type]`
- `init` - Initializes the `SubStates` with default values
  Same as having `#[auto_init_sub_state]`

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_states(plugin = MyPlugin, derive, init)]
enum MainState {
    #[default]
    InGame,
    Menu,
}

#[auto_sub_states(plugin = MyPlugin, derive, init)]
#[source(MainState = MainState::InGame)]
enum InGameState {
    #[default]
    Playing,
    Paused,
}
```
