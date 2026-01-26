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

#[auto_resource(plugin = Test, derive(Debug, Default, PartialEq), reflect, register, init)]
struct FooResource(usize);

#[auto_resource(plugin = Test, generics(usize), derive(Debug, Default, PartialEq), reflect, register, init, insert(GenericResource(true)))]
struct GenericResource<T>(T);

#[auto_resource(plugin = Test, derive(Debug, PartialEq), insert(InsertedResource(42)))]
struct InsertedResource(usize);

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_register_type_foo_resource() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooResource>()),
        "did not auto register resource type"
    );
}

#[xtest]
fn test_auto_init_resource() {
    let app = app();
    assert!(app.world().contains_resource::<FooResource>());
    assert_eq!(app.world().resource::<FooResource>().0, 0);
}

#[xtest]
fn test_auto_init_generic_resource() {
    let app = app();
    assert!(app.world().contains_resource::<GenericResource<usize>>());
    assert_eq!(app.world().resource::<GenericResource<usize>>().0, 0);
}

#[xtest]
fn test_auto_insert_generic_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<GenericResource<bool>>(),
        Some(&GenericResource(true)),
        "did not auto insert resource"
    );
}

#[xtest]
fn test_auto_insert_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<InsertedResource>(),
        Some(&InsertedResource(42)),
        "did not auto insert resource"
    );
}
