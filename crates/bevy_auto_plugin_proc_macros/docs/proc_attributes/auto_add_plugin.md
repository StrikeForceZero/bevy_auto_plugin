Automatically inserts a plugin as a sub-plugin into the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should add this plugin.
- `value(Value)` - Optional. Specifies the plugin value to insert.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the resource will be inserted with these specific generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(AutoPlugin, Default)]
#[auto_plugin(impl_plugin_trait)]
#[auto_add_plugin(plugin = MyPlugin)]
struct MySubPlugin;
```

# Example (with value)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(AutoPlugin, Default)]
#[auto_plugin(impl_plugin_trait)]
#[auto_add_plugin(plugin = MyPlugin, value(42))]
struct MySubPlugin(usize);
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(AutoPlugin, Default)]
#[auto_plugin(impl_plugin_trait)]
#[auto_add_plugin(plugin = MyPlugin, generics(usize))]
struct FooResourceWithGeneric<T>(T);
```