# Breaking Changes / Migration Guides

## v0.2.x -> 0.3

### bevy_auto_plugin::auto_plugin::*, bevy_auto_plugin::auto_plugin_module::*

- `use bevy_auto_plugin::flat_file::prelude::*`
- `use bevy_auto_plugin::module::prelude::*`

## v0.3 -> v0.4

### `#[auto_*(*<*>)]` -> `#[auto_*(generics(*))]`

```regexp
/(auto_\w+)\(\w+<(.*?)>\)/\1(generics(\2))/
```

### Enable `flat_file`, `module` modes

`features = [mode_flat_file, mode_module]` or `features = [all_modes]`

### Updated Imports
- `use bevy_auto_plugin::flat_file::prelude::*` -> `use bevy_auto_plugin::modes::flat_file::prelude::*`
- `use bevy_auto_plugin::module::prelude::*` -> `use bevy_auto_plugin::modes::module::prelude::*`