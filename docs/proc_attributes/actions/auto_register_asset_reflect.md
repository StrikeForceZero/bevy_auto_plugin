Automatically registers an asset for Bevy asset reflection.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should register this asset reflection.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, asset reflection will be registered with these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(T1 = ..., T2 = ...)`.

# Notes
- This attribute can be applied to a `use` item; each imported name becomes its own target.
- `use ...::*`, `use ...::self`, and `_` imports are not supported.
- Renames (`as`) are supported and use the local name.
- This calls Bevy's `register_asset_reflect`, which registers the asset type and handle type, and adds asset-specific reflection data.
- The target type must satisfy Bevy's `register_asset_reflect` bounds, usually by deriving `Asset` and `Reflect`.

# Example
```rust
use bevy::prelude::*;
use bevy_asset::Asset;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Asset, Reflect)]
#[auto_register_asset_reflect(plugin = MyPlugin)]
struct FooAsset;
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_asset::Asset;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Asset, Reflect)]
#[auto_register_asset_reflect(plugin = MyPlugin, generics(usize))]
struct FooAssetWithGeneric<T: Reflect + TypePath + Send + Sync + 'static>(T);
```
