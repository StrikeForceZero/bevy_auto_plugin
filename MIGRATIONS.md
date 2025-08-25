# Breaking Changes / Migration Guides

## v0.2.x to 0.3

### Updated Imports

- `bevy_auto_plugin::auto_plugin::*` -> `use bevy_auto_plugin::flat_file::prelude::*`
- `bevy_auto_plugin::auto_plugin_module::*` -> `use bevy_auto_plugin::module::prelude::*`

## v0.3 to v0.4

### Specifying generics now uses `generics(T1, T2, ..)`
`#[auto_*(*<*>)]` -> `#[auto_*(generics(*))]`
- enable `legacy_path_param` until you have replaced all usages
- find replace regex:
```regexp
/(auto_\w+)\(\w+<(.*?)>\)/\1(generics(\2))/
```
- for some attributes you can now specify multiple sets `#[auto_(generics(u8, bool), generics(u8, u32))]`

### Enable `flat_file`, `module` modes
- by default flat_file and module modes are not enabled, if you use them enable their respective features:
`features = ["mode_flat_file", "mode_module"]` or `features = ["all_modes"]`
- if you were using the flat_file mode, you might also need to enable `flat_file_lang_server_noop` feature as it's no longer enabled by default

### Updated Imports
- `use bevy_auto_plugin::flat_file::prelude::*` -> `use bevy_auto_plugin::modes::flat_file::prelude::*`
- `use bevy_auto_plugin::module::prelude::*` -> `use bevy_auto_plugin::modes::module::prelude::*`

## v0.4 to v0.5

### `global_auto_plugin(..)` is now just `auto_plugin(..)`

### `auto_plugin(app=..)` is now `auto_plugin(app_param=..)` across all modes

### `flat_file` and `module` modes are deprecated 
  - [see feedback issue if you are using these modes](https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19) or use `global`
  - enable `ignore_flat_file_or_module_deprecation` to disable deprecation warnings if you commented and provided a use case for keeping the modes