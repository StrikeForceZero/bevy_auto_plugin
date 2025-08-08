A derive macro that implements Plugin for a struct and collects registered components,
events, resources, and systems.

# Parameters
- `impl_plugin_trait` - Optional. When present, automatically implements the Plugin trait.
- `impl_generic_plugin_trait` - Optional. When present, automatically implements the Plugin trait universally across all generics.
- `impl_generic_auto_plugin_trait` - Optional. When present, automatically implements the AutoPlugin trait universally across all generics.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

// Plugin will automatically implement the Plugin trait
// and include all registered components, events, resources, etc.
```