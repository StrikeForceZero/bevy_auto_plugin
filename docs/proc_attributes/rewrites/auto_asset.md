Automatically derives `Asset` with `TypePath` or `Reflect`, registers asset reflection, and initializes the asset in the app.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should initialize this asset.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the `register` and `init` actions use these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.
- `derive` | `derive(Debug, Default, ..)` - Optional. Specifies that the macro should handle deriving `Asset` and `TypePath`.
  When `reflect` is also enabled, this derives `Asset` and `Reflect`; `Reflect` supplies the required `TypePath` implementation.
  Passes through any additional derives listed.
- `reflect` | `reflect(Debug, Default, ..)` - Optional. Specifies that the macro should handle emitting the single `#[reflect(...)]`.
  Passes through any additional reflects listed.
  If enabled in tandem with `derive` it also includes `#[derive(Reflect)]`.
- `register` - Registers the asset for Bevy asset reflection by calling Bevy's `register_asset_reflect`.
  Same as having `#[auto_register_asset_reflect]`.
  The target type must satisfy Bevy's `register_asset_reflect` bounds, usually by deriving `Asset` and `Reflect`.
- `init` - Initializes the asset in the app.
  Same as having `#[auto_init_asset]`.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_asset(plugin = MyPlugin, derive(Debug), init)]
struct FooAsset;
```

# Example (with reflect)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_asset(plugin = MyPlugin, derive(Debug), reflect(Debug), register, init)]
struct ReflectedAsset {
    value: usize,
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_reflect::TypePath;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_asset(plugin = MyPlugin, generics(usize), derive(Debug), init)]
struct FooAssetWithGeneric<T: TypePath + Send + Sync + 'static>(T);
```
