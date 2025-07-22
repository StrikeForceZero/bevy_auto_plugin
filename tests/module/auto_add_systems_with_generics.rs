use bevy_app::prelude::*;
use bevy_auto_plugin::module::prelude::*;
use bevy_ecs::prelude::*;
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

#[test]
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
}
