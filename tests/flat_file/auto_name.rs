use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::name::Name;
use bevy_ecs::prelude::*;

#[derive(Component)]
#[auto_name]
pub struct Test;

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
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
