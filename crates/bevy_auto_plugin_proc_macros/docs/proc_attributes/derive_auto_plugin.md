A derive macro that implements Plugin for a struct and collects registered components,
events, resources, and systems.

# Parameters
- `impl_plugin_trait` - Optional. When present, it automatically implements the Plugin trait.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

// Plugin will automatically implement the Plugin trait
// and include all registered components, events, resources, etc.
```