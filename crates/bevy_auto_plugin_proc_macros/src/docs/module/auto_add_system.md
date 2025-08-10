Automatically registers a system to be added to the app in module mode.

# Parameters
- `schedule = ScheduleName` - Required. Specifies which schedule to add the system to.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
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

# Example (basic)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[derive(Resource, Debug, Default)]
    pub(super) struct TestResource(pub i32);

    #[auto_add_system(schedule = Update)]
    pub(super) fn test_system(mut res: ResMut<TestResource>) {
        res.0 += 1;
    }
}

fn plugin(app: &mut App) {
    app.init_resource::<my_plugin::TestResource>();
    app.add_plugins(my_plugin::init);
}
```

# Example (with system set)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
pub mod my_plugin {
    use bevy::prelude::*;
    use bevy_auto_plugin::modes::module::prelude::*;

    #[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub(super) enum TestSet {
        First,
    }

    #[derive(Resource, Debug, Default)]
    pub(super) struct TestResource(pub i32);

    #[auto_add_system(schedule = Update, config(in_set = TestSet::First))]
    pub(super) fn test_system(mut res: ResMut<TestResource>) {
        res.0 += 1;
    }
}

fn plugin(app: &mut App) {
    app.init_resource::<my_plugin::TestResource>();
    app.configure_sets(Update, my_plugin::TestSet::First);
    app.add_plugins(my_plugin::init);
}
```