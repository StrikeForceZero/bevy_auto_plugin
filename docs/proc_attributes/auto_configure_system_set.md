Automatically configures a SystemSet for the app.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should register this system.
- `schedule = ScheduleName` - Required. Specifies which schedule to add the system to.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
- `group` - Optional. Specifies what group this config is for. Omitting acts like a group.
  - it's recommended to use the schedule label as your group key. e.g. `Update` or `FixedUpdate`
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
- `chain` - Optional. calls `.chain()` on the resultant set.
- `chain_ignore_deferred` - Optional. calls `.chain_ignore_deferred()` on the resultant set.
- `config(..)`
  - `in_set = SetName` - Optional. See [`bevy IntoScheduleConfigs in_set`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.in_set)
  - `before = SetName or system` - Optional. See [`bevy IntoScheduleConfigs before`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.before)
  - `after = SetName or system` - Optional. See [`bevy IntoScheduleConfigs after`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.after)
  - `run_if = Condition` - Optional. See [`bevy IntoScheduleConfigs run_if`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.run_if)
  - `distributive_run_if = Condition` - Optional. See [`bevy IntoScheduleConfigs run_if_inner`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.run_if_inner)
  - `ambiguous_with = System` - Optional. See [`bevy IntoScheduleConfigs ambiguous_with`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with)
  - `ambiguous_with_all = bool` - Optional. See [`bevy IntoScheduleConfigs ambiguous_with_all`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with_all)
  - `after_ignore_deferred = SetName or system` - Optional. See [`bevy IntoScheduleConfigs after_ignore_deferred`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.after_ignore_deferred)
  - `before_ignore_deferred = SetName or system` - Optional. See [`bevy IntoScheduleConfigs before_ignore_deferred`](https://docs.rs/bevy/0.16.1/bevy/prelude/trait.IntoScheduleConfigs.html#method.before_ignore_deferred)

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_configure_system_set(plugin = MyPlugin, schedule = Update)]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MySet;
```

# Example Enum 

See [auto_configure_system_set_config](auto_configure_system_set_config.md) for details.

```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_configure_system_set(plugin = MyPlugin, schedule = Update)]
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MySet {
    #[auto_configure_system_set_config(config(run_if = || false))]
    A,
    B,
}
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum TestSet { First, Second }

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_configure_system_set(plugin = MyPlugin, generics(usize), schedule = Update)]
#[derive(SystemSet, Debug, Default, Hash, PartialEq, Eq, Clone)]
struct MySet<T>(T);
```