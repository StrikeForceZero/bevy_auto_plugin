Automatically inserts a plugin as a sub-plugin into the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should add this plugin.
- `init | init(SubPluginValue) | init = SubPluginValue` - Optional.
  - ` ` for unit struct sub-plugins. e.g. `YourSubPlugin`
  - `init` for sub-plugins deriving `Default`. e.g. `YourSubPlugin::default()` 
  - `init = ... | init(...)` for custom values of the sub-plugin. e.g. `YourSubPlugin("data")` or `YourSubPlugin { foo: "bar" }`
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, along with impl_plugin_trait, the plugin will be derived with these specific generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
#[auto_add_plugin(plugin = MyPlugin)]
struct MySubPlugin;
```

# Example (with default value)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(AutoPlugin, Default)]
#[auto_plugin(impl_plugin_trait)]
#[auto_add_plugin(plugin = MyPlugin, init)]
struct MySubPlugin(usize);
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
#[auto_add_plugin(plugin = MyPlugin, init = MySubPlugin(42))]
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
#[auto_add_plugin(plugin = MyPlugin, generics(usize), init)]
struct FooResourceWithGeneric<T>(T);
```