Automatically initializes a state in the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should initialize this state.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the states will be registered with these specific generic parameters.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `States`.
  Passes through any additional derives listed.
  When enabled, `States` include these additional derives:  
    - `Debug`
    - `Default`
    - `Copy`
    - `Clone`
    - `PartialEq`
    - `Eq`
    - `Hash`
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]`
- `register` - Enables type registration for the `States`
  Same as having `#[auto_register_type]`
- `init` - Initializes the `States` with default values
  Same as having `#[auto_init_state]`

// Debug, Default, Copy, Clone, PartialEq, Eq, Hash

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_states(plugin = MyPlugin, derive, reflect, register, init)]
enum FooState {
    #[default]
    Start,
    End,
}
```