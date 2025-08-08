use bevy_app::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;
use std::fmt::Debug;
use std::ops::{Add, AddAssign};

trait One {
    const ONE: Self;
}

impl One for u32 {
    const ONE: Self = 1;
}

impl One for i32 {
    const ONE: Self = 1;
}

trait TestNumber<T = Self>:
    One + Debug + Default + PartialEq + PartialOrd + Add<Output = T> + AddAssign + Send + Sync
{
}

impl TestNumber for u32 {}
impl TestNumber for i32 {}

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;
    use bevy_ecs::prelude::*;

    #[derive(Resource, Debug, Default, PartialEq)]
    pub(super) struct Test<T>(pub T)
    where
        T: TestNumber<T> + 'static;

    #[auto_add_system(schedule = Update, generics(u32))]
    #[auto_add_system(schedule = Update, generics(i32))]
    pub(super) fn foo_system<T>(mut test: ResMut<Test<T>>)
    where
        T: TestNumber<T> + 'static,
    {
        test.0 += T::ONE;
    }

    pub(super) fn run_if_gt_2<T>(test: Res<Test<T>>) -> bool
    where
        T: TestNumber<T> + 'static,
    {
        let two = T::ONE + T::ONE;
        test.0 > two
    }

    #[auto_add_system(schedule = Update, generics(u32), run_if = run_if_gt_2::<u32>, before = foo_system::<u32>)]
    #[auto_add_system(schedule = Update, generics(i32), run_if = run_if_gt_2::<i32>, before = foo_system::<i32>)]
    pub(super) fn bar_system<T>(mut test: ResMut<Test<T>>)
    where
        T: TestNumber<T> + 'static,
    {
        test.0 += T::ONE;
    }

    pub(super) fn plugin(app: &mut App) {
        app.init_resource::<Test<u32>>();
        app.init_resource::<Test<i32>>();
        init(app);
    }
}

use plugin_module::*;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

fn test_eq<T>(app: &App, b: T)
where
    T: TestNumber<T> + 'static,
{
    assert_eq!(app.world().resource::<Test<T>>(), &Test(b));
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_systems() {
    let mut app = app();
    test_eq(&app, 0u32);
    test_eq(&app, 0i32);
    app.update();
    test_eq(&app, 1u32);
    test_eq(&app, 1i32);
    app.update();
    test_eq(&app, 2u32);
    test_eq(&app, 2i32);
    // without before run_if would trigger here
    app.update();
    test_eq(&app, 3u32);
    test_eq(&app, 3i32);
    // run_if triggered
    app.update();
    test_eq(&app, 5u32);
    test_eq(&app, 5i32);
}
