Attribute to mark the build function for the plugin, or impl Plugin trait build method for injection

# Parameters
- `plugin = PluginType` - **Required for bare functions only.** Specifies the plugin this build function belongs to.  
  **Not allowed on `impl Plugin` methods**, since the plugin type is already known.
- `app_param = identifier` - *(Optional)* Specifies the name of the `App` parameter that code will be injected into.  
  Defaults to `app` if omitted.

# Example - impl Plugin
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
struct MyPlugin;

impl Plugin for MyPlugin {
    #[auto_plugin(app_param=non_default_app_param_name)]
    fn build(&self, non_default_app_param_name: &mut App) {
        // code injected here

        // your code
    }
}
```

# Example - bare fn
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
struct MyPlugin;

#[auto_plugin(plugin = MyPlugin, app_param=non_default_app_param_name)]
fn build(non_default_app_param_name: &mut App) {
    // code injected here

    // your code
}
```