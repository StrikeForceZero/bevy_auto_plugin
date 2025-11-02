Automatically configures a SystemSet for the app.

# Parameters
- `group` - Optional. Specifies what group this config is for. Omitting acts like default for all groups.
  - it's recommended to use the schedule label as your group key. e.g. `Update` or `FixedUpdate`
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

fn always() -> bool {
    true
}

fn never() -> bool {
    true
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[auto_configure_system_set(plugin = MyPlugin, group = Update, schedule = Update)]
#[auto_configure_system_set(plugin = MyPlugin, group = FixedUpdate, schedule = FixedUpdate)]
enum MySet {
    A,
    #[auto_configure_system_set_config(config(run_if = never))]
    B,
    #[auto_configure_system_set_config(group = Update, config(run_if = never))]
    #[auto_configure_system_set_config(group = FixedUpdate, config(run_if = always))]
    C,
}

/* RESULT:
app.configure_sets(Update, (MySet::A, MySet::B.run_if(never), MySet::C.run_if(never));
app.configure_sets(FixedUpdate, (MySet::A, MySet::B.run_if(never), MySet::C.run_if(always));
*/
```