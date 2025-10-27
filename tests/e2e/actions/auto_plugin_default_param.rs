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
    }
}

#[derive(Resource, Debug, PartialEq)]
struct Test(usize);

impl Default for Test {
    fn default() -> Self {
        Self(1)
    }
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_plugin_param() {
    let app = app();
    assert_eq!(app.world().get_resource::<Test>(), Some(&Test(1)));
}
