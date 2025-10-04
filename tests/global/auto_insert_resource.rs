use bevy_app::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_ecs::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_resource(plugin = TestPlugin)]
#[auto_insert_resource(plugin = TestPlugin, resource(Test(1)))]
#[derive(Resource, Debug, Default, PartialEq)]
struct Test(usize);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test>(),
        Some(&Test(1)),
        "did not auto insert resource"
    );
}
