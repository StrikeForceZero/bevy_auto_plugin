Automatically registers an message to be added to the app in global mode.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this message.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the message will be registered with these specific generic parameters.

# Example

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_add_message(plugin = MyPlugin)]
struct FooMessage(usize);
```

# Example (with generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = MyPlugin, generics(usize))]
#[auto_add_message(plugin = MyPlugin, generics(usize))]
struct FooMessageWithGeneric<T>(T);
```