#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_proc_macro::xtest;
use internal_test_util::{
    create_minimal_app,
    type_id_of,
};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_component(plugin = Test, derive, register, reflect, auto_name)]
struct FooComponent;

#[auto_component(plugin = Test, generics(usize), derive, register, reflect, auto_name)]
struct GenericComponent<T>(T);

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_register_type_foo_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<FooComponent>()), "did not auto register type");
}

#[xtest]
fn test_auto_register_type_generic_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<GenericComponent<usize>>()),
        "did not auto register generic type"
    );
}

#[xtest]
fn test_auto_name_foo_component() {
    let mut app = app();
    app.world_mut().spawn(FooComponent);
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<FooComponent>>()
        .single(app.world())
        .expect("failed to query FooComponent");
    assert_eq!(name, &Name::new("FooComponent"), "did not auto name FooComponent");
}

#[xtest]
fn test_auto_name_generic_component() {
    let mut app = app();
    app.world_mut().spawn(GenericComponent(10usize));
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<GenericComponent<usize>>>()
        .single(app.world())
        .expect("failed to query GenericComponent");
    assert_eq!(name, &Name::new("GenericComponent<usize>"), "did not auto name GenericComponent");
}
