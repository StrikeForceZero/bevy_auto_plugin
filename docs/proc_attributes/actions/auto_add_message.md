Automatically registers an message to be added to the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this message.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the message will be registered with these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(T1 = ..., T2 = ...)`.
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

#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_add_message(plugin = MyPlugin)]
struct FooMessage(usize);
```

# Example (with generics)

```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = MyPlugin, generics(usize))]
#[auto_add_message(plugin = MyPlugin, generics(usize))]
struct FooMessageWithGeneric<T>(T);
```