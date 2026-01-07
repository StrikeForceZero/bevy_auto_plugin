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

#[auto_message(plugin = Test, derive(Debug, Default, PartialEq), reflect, register)]
struct FooMessage(usize);

#[auto_message(plugin = Test, generics(usize), derive(Debug, Default, PartialEq), reflect, register)]
struct GenericMessage<T>(T);

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_register_type_foo_message() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooMessage>()),
        "did not auto register message type"
    );
}

#[xtest]
fn test_auto_register_type_generic_message() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<GenericMessage<usize>>()),
        "did not auto register generic message type"
    );
}
