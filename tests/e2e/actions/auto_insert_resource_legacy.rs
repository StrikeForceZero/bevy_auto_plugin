use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_insert_resource(plugin = TestPlugin, init(LegacyInit(1)))]
#[derive(Resource, Debug, PartialEq)]
struct LegacyInit(usize);

#[auto_insert_resource(plugin = TestPlugin, resource(LegacyResource(2)))]
#[derive(Resource, Debug, PartialEq)]
struct LegacyResource(usize);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_insert_resource_legacy() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<LegacyInit>(),
        Some(&LegacyInit(1)),
        "did not auto insert legacy init resource",
    );
    assert_eq!(
        app.world().get_resource::<LegacyResource>(),
        Some(&LegacyResource(2)),
        "did not auto insert legacy resource",
    );
}
