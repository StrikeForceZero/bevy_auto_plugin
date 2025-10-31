use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::{
    app::StatesPlugin,
    prelude::*,
};
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_state(plugin = TestPlugin)]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
enum Test {
    #[default]
    A,
    #[allow(dead_code)]
    B,
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_init_state() {
    let app = app();
    assert!(app.world().get_resource::<State<Test>>().is_some(), "did not auto init state");
    assert!(app.world().get_resource::<NextState<Test>>().is_some(), "did not auto init state");
}
