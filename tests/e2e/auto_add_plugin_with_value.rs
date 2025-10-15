use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(AutoPlugin)]
#[auto_add_plugin(plugin = TestPlugin, value = TestSubPlugin(1))]
struct TestSubPlugin(usize);

impl Plugin for TestSubPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Test(self.0));
    }
}

#[derive(Resource, Debug, Default, PartialEq)]
struct Test(usize);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_add_plugin() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test>(),
        Some(&Test(1)),
        "did not auto add plugin"
    );
}
