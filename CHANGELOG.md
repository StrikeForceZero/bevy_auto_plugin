# Changelog

## v0.1

#### Public release
- auto_plugin (requires nightly)
- auto_plugin_module
- bevy 0.15
- add `#[auto_plugin(..)]`
- add `#[auto_register_type(..)]`
- add `#[auto_name(..)]`
- add `#[auto_init_resource(..)]`
- add `#[auto_add_event(..)]`

---
## v0.1.1 (yanked for bad dep versioning)

---
## v0.1.2
- add `#[auto_register_state_type(..)]`
- add `#[auto_init_state(..)]`

---
## v0.2

- bevy 0.16

---
## v0.2.1

- add feature to use older nightly api `nightly-2025-04-15`

#### Updated Imports

- `bevy_auto_plugin::auto_plugin::*` -> `use bevy_auto_plugin::flat_file::prelude::*`
- `bevy_auto_plugin::auto_plugin_module::*` -> `use bevy_auto_plugin::module::prelude::*`

---
## v0.3

- remove nightly requirements
- require rust >=1.88

---
## v0.4

- add `#[auto_insert_resource(..)]`
- add `#[auto_add_observer(..)]`
- add `#[auto_add_system(..)]`
- add `global` mode
  - add `derive(AutoPlugin)`
  - add `#[global_auto_plugin(..)]`

#### Specifying generics now uses `generics(T1, T2, ..)`
- enable `legacy_path_param` until you have replaced all usages

#### Enable `flat_file`, `module` modes
- by default, flat_file and module modes are not enabled; if you use them enable their respective features:
  `features = ["mode_flat_file", "mode_module"]` or `features = ["all_modes"]`
- if you were using the flat_file mode, you might also need to enable `flat_file_lang_server_noop` feature as it's no longer enabled by default

#### Updated Imports
- `use bevy_auto_plugin::flat_file::prelude::*` -> `use bevy_auto_plugin::modes::flat_file::prelude::*`
- `use bevy_auto_plugin::module::prelude::*` -> `use bevy_auto_plugin::modes::module::prelude::*`

---
## v0.5
- remove deprecated pre-v0.4 module exports
- deprecate `flat_file` and `module` modes ([see feedback issue if you are using these modes](https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19))
- add shorthand macros
  - `#[auto_component(..)]` 
  - `#[auto_resource(..)]` 
  - `#[auto_event(..)]` 
  - `#[auto_observer(..)]` 
  - `#[auto_system(..)]`
  - `#[auto_states(..)]` 
  - `#[auto_bind_plugin(..)]`

#### `global_auto_plugin(..)` is now just `auto_plugin(..)`

#### `auto_plugin(app=..)` is now `auto_plugin(app_param=..)` across all modes

---
### v0.6
- update to bevy 0.17
- add `auto_add_message` to register items deriving `Message` on plugin build
- add `auto_message` shorthand to derive `Message` and add to bevy app
- update `auto_event` with new params `target(global)` or `target(entity)`
- deprecated `auto_add_event` (aliased to `auto_add_message`)
- fix `auto_component` not passing generics to `auto_name`

---
### v0.6.1
- Add support for `auto_system` param `schedule` to accept `ExprCall` in addition to `ExprPath`

---
### v0.6.2 (yanked for bad dep versioning)

---
### v0.6.3
- Fix compat issue by allowing more than one `ScheduleConfig` condition
- Fix compat issue by allow more than `Path` for `ScheduleConfig`

---
### v0.7.0
- Add `name` param to `auto_name` attribute
- Add the ability to set a custom name with `auto_name` param in `auto_component`
- Add `auto_run_on_build` to run functions and pass `&mut bevy_app::App as the only param`
- Expanded `auto_insert_resource` `resource` to accept more potentially valid expressions
- Remove re-exported crates `bevy_ecs`, `bevy_ecs_macros`, `bevy_state`, `bevy_log`, `bevy_reflect`, `bevy_reflect_derive`
  - Fixes 
    > no method named `with_docs` found for struct `type_info::OpaqueInfo` in the current scope
- Adds feature `debug_log_plugin_registry` to gate logging 
    > Building AutoPluginRegistry from {count} entries
- Change exports from `use bevy_auto_plugin::modes::global::prelude::*` -> `use bevy_auto_plugin::prelude::*`

---
### v0.7.1
- Restore and deprecate old imports `use bevy_auto_plugin::modes::global::prelude::*` -> `use bevy_auto_plugin::prelude::*`
 