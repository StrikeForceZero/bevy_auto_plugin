use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_resource(plugin = TestPlugin, generics(usize, bool))]
#[auto_insert_resource(plugin = TestPlugin, generics(usize, bool), resource(Test(1, true)))]
#[derive(Resource, Debug, Default, PartialEq)]
struct Test<T1, T2>(T1, T2);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test::<usize, bool>>(),
        Some(&Test(1, true)),
        "did not auto insert resource"
    );
}
