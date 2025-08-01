use bevy_app::prelude::*;
use bevy_auto_plugin::global::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use internal_test_util::{create_minimal_app, type_id_of};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test)]
#[auto_name(plugin = Test)]
struct FooComponent;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test)]
#[auto_init_resource(plugin = Test)]
struct FooRes(usize);

#[auto_add_system(plugin = Test, schedule = Update)]
fn foo_system(mut foo_res: ResMut<FooRes>) {
    foo_res.0 += 1;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[test]
fn test_auto_register_type_foo_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooComponent>()),
        "did not auto register type"
    );
}

#[test]
fn test_auto_name_foo_component() {
    let mut app = app();
    app.world_mut().spawn(FooComponent);
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<FooComponent>>()
        .single(app.world())
        .expect("failed to query FooComponent");
    assert_eq!(
        name,
        &Name::new("FooComponent"),
        "did not auto name FooComponent"
    );
}

#[test]
fn test_auto_init_resource_foo_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(0)),
        "did not auto init resource"
    );
}

#[test]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(0)),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(1)),
        "did not register system"
    );
}
