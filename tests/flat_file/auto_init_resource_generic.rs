use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[auto_init_resource(generics(bool))]
#[derive(Resource, Default)]
struct Test<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_init_resource_generic() {
    let app = app();
    assert!(
        app.world().get_resource::<Test<bool>>().is_some(),
        "did not auto init resource"
    );
}
