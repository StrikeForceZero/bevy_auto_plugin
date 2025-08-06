use bevy_app::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use internal_test_util::type_id_of;

#[auto_register_type(Test<bool>)]
#[derive(Reflect)]
struct Test<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_type_generic() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<Test<bool>>()),
        "did not auto register type"
    );
}
