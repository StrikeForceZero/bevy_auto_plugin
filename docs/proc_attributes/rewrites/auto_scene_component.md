Automatically registers a scene component to be added to the app.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should register this scene component.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the scene component will be registered with these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.
- `derive` | `derive(Default, Clone, ..)` - Optional. Specifies that the macro should handle deriving `SceneComponent`.
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]`.
  Scene components are reflected as components, so this emits `#[reflect(Component, ...)]`.
- `register` - Enables type registration for the scene component.
  Same as having `#[auto_register_type]`.
- `auto_name | auto_name = ...` - Enables adding a required component of `Name` with the scene component's concrete name or custom name literal if specified.
  Same as having `#[auto_name]` or `#[auto_name = ...]`.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_scene_component(
    plugin = MyPlugin,
    derive(Default, Clone),
    reflect,
    register,
    auto_name,
)]
#[scene("player.bsn")]
struct Player {
    score: usize,
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_scene_component(plugin = MyPlugin, generics(usize), derive(Default, Clone), reflect, register)]
#[scene("slot.bsn")]
struct Slot<T: Default + Clone + Unpin + Reflect>(T);
```
