use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::type_id_of;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait, default_plugin)]
struct DefaultPlugin;

#[auto_register_type]
#[derive(Reflect)]
struct DefaultedType;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(DefaultPlugin);
    app
}

#[xtest]
fn test_auto_default_plugin_register_type() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<DefaultedType>()),
        "did not auto register DefaultedType"
    );
}
