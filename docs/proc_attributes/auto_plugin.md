Attribute to mark the build function for the plugin, or impl Plugin trait build method for injection

# Parameters
- `plugin = PluginType` - **Required for bare functions only.** Specifies the plugin this build function belongs to.  
  **Not allowed on `impl Plugin` methods**, since the plugin type is already known.

# Example - impl Plugin
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
struct MyPlugin;

impl Plugin for MyPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
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

#[auto_plugin(plugin = MyPlugin)]
fn build(app: &mut App) {
    // code injected here

    // your code
}
```