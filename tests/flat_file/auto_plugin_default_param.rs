use bevy_app::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use internal_test_util::type_id_of;

#[auto_register_type]
#[derive(Reflect)]
struct Test;

#[auto_plugin]
fn plugin(my_app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_plugin_param() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<Test>()),
        "did not auto register type"
    );
}
