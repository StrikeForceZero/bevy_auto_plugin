use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_resource(plugin = TestPlugin)]
#[derive(Resource, Default)]
struct Test;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_init_resource() {
    let app = app();
    assert!(
        app.world().get_resource::<Test>().is_some(),
        "did not auto init resource"
    );
}
