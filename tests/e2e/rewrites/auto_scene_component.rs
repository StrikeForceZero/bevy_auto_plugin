#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::{
    create_minimal_app,
    type_id_of,
};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_scene_component(plugin = Test, derive(Default, Clone), register, reflect, auto_name)]
#[scene("foo.bsn")]
struct FooSceneComponent;

#[auto_scene_component(plugin = Test, generics(usize), derive(Default, Clone), register, reflect, auto_name)]
#[scene("generic.bsn")]
struct GenericSceneComponent<T: Default + Clone + Unpin + Reflect>(T);

#[derive(SceneComponent, Reflect, Default, Clone)]
#[auto_scene_component(plugin = Test, register, reflect, auto_name)]
#[scene("manual.bsn")]
struct ManualDerivedSceneComponent;

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_register_type_foo_scene_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooSceneComponent>()),
        "did not auto register scene component type"
    );
}

#[xtest]
fn test_auto_register_type_generic_scene_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<GenericSceneComponent<usize>>()),
        "did not auto register generic scene component type"
    );
}

#[xtest]
fn test_auto_register_type_manual_derived_scene_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<ManualDerivedSceneComponent>()),
        "did not auto register manually derived scene component type"
    );
}

#[xtest]
fn test_auto_name_foo_scene_component() {
    let mut app = app();
    app.world_mut().spawn(FooSceneComponent);
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<FooSceneComponent>>()
        .single(app.world())
        .expect("failed to query FooSceneComponent");
    assert_eq!(name, &Name::new("FooSceneComponent"), "did not auto name FooSceneComponent");
}

#[xtest]
fn test_auto_name_generic_scene_component() {
    let mut app = app();
    app.world_mut().spawn(GenericSceneComponent(10usize));
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<GenericSceneComponent<usize>>>()
        .single(app.world())
        .expect("failed to query GenericSceneComponent");
    assert_eq!(
        name,
        &Name::new("GenericSceneComponent<usize>"),
        "did not auto name GenericSceneComponent"
    );
}

#[xtest]
fn test_auto_name_manual_derived_scene_component() {
    let mut app = app();
    app.world_mut().spawn(ManualDerivedSceneComponent);
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<ManualDerivedSceneComponent>>()
        .single(app.world())
        .expect("failed to query ManualDerivedSceneComponent");
    assert_eq!(
        name,
        &Name::new("ManualDerivedSceneComponent"),
        "did not auto name ManualDerivedSceneComponent"
    );
}
