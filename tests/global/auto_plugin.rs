use bevy_app::prelude::*;
use bevy_auto_plugin::global::prelude::{AutoPlugin, auto_register_type};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use internal_test_util::{create_minimal_app, type_id_of};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test)]
struct Foo;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test)]
struct FooRes(usize);

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[test]
fn test_auto_register_type() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<Foo>()),
        "did not auto register type"
    );
}
