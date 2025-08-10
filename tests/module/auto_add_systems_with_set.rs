use bevy_app::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;
use bevy_ecs::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;
    #[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub(super) enum TestSet {
        First,
        Second,
        Third,
    }

    #[derive(Resource, Debug, Clone, Default, PartialEq)]
    pub(super) struct Test(pub Vec<&'static str>);

    #[auto_add_system(schedule = Update, config(in_set = TestSet::First))]
    pub(super) fn a_system(mut test: ResMut<Test>) {
        test.0.push("a");
    }
    #[auto_add_system(schedule = Update, config(in_set = TestSet::Second))]
    pub(super) fn b_system(mut test: ResMut<Test>) {
        test.0.push("b");
    }
    #[auto_add_system(schedule = Update, config(in_set = TestSet::Third))]
    pub(super) fn c_system(mut test: ResMut<Test>) {
        test.0.push("c");
    }

    pub(super) fn plugin(app: &mut App) {
        app.init_resource::<Test>();
        app.configure_sets(
            Update,
            (TestSet::First, TestSet::Second, TestSet::Third).chain(),
        );
        init(app);
    }
}

use plugin_module::*;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_systems() {
    let mut app = app();
    app.update();
    assert_eq!(app.world().resource::<Test>(), &Test(vec!["a", "b", "c"]));
}
