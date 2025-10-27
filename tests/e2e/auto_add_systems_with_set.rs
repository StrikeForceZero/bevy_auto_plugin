use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
struct TestPlugin;

impl Plugin for TestPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.init_resource::<Test>();
        app.configure_sets(Update, (TestSet::First, TestSet::Second, TestSet::Third).chain());
    }
}

#[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TestSet {
    First,
    Second,
    Third,
}

#[derive(Resource, Debug, Clone, Default, PartialEq)]
struct Test(Vec<&'static str>);

#[auto_add_system(plugin = TestPlugin, schedule = Update, config(in_set = TestSet::First))]
fn a_system(mut test: ResMut<Test>) {
    test.0.push("a");
}
#[auto_add_system(plugin = TestPlugin, schedule = Update, config(in_set = TestSet::Second))]
fn b_system(mut test: ResMut<Test>) {
    test.0.push("b");
}
#[auto_add_system(plugin = TestPlugin, schedule = Update, config(in_set = TestSet::Third))]
fn c_system(mut test: ResMut<Test>) {
    test.0.push("c");
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_register_systems() {
    let mut app = app();
    app.update();
    assert_eq!(app.world().resource::<Test>(), &Test(vec!["a", "b", "c"]));
}
