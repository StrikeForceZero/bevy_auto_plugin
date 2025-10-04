use bevy_app::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_ecs::name::Name;
use bevy_ecs::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(Component)]
#[auto_name(plugin = TestPlugin)]
pub struct Test;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_name() {
    let mut app = app();
    let entity = app.world_mut().spawn(Test).id();
    app.update();
    assert_eq!(
        app.world().entity(entity).get::<Name>(),
        Some(&Name::new("Test"))
    );
}
