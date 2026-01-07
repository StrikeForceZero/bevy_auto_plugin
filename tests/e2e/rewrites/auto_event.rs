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

#[auto_event(plugin = Test, target(global), derive(Debug, Default, PartialEq), reflect, register)]
struct FooGlobalEvent(usize);

#[auto_event(plugin = Test, target(entity), derive(Debug, PartialEq), reflect, register)]
struct FooEntityEvent(#[event_target] Entity);

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_register_type_foo_global_event() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooGlobalEvent>()),
        "did not auto register global event type"
    );
}

#[xtest]
fn test_auto_register_type_foo_entity_event() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooEntityEvent>()),
        "did not auto register entity event type"
    );
}

#[xtest]
fn test_global_event_trigger() {
    let mut app = app();
    app.world_mut().trigger(FooGlobalEvent(42));
}

#[xtest]
fn test_entity_event_trigger() {
    let mut app = app();
    let entity = app.world_mut().spawn_empty().id();
    app.world_mut().trigger(FooEntityEvent(entity));
}
