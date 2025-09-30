#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_util::{create_minimal_app, type_id_of};
use std::ops::Deref;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_bind_plugin(plugin = Test)]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type]
#[auto_name]
struct FooComponent;

#[auto_bind_plugin(plugin = Test)]
#[auto_component(derive, register, reflect, auto_name)]
struct FooComponent2;

#[auto_bind_plugin(plugin = Test)]
#[auto_component(derive, register, reflect, auto_name)]
struct FooComponent3;

#[auto_bind_plugin(plugin = Test)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
struct FooDefaultRes(usize);

#[auto_bind_plugin(plugin = Test)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
#[auto_insert_resource(resource(FooRes(1)))]
struct FooRes(usize);

#[auto_bind_plugin(plugin = Test)]
#[auto_resource(derive, register, reflect, init)]
#[derive(Default)]
struct FooRes2(usize);

#[auto_bind_plugin(plugin = Test)]
#[auto_resource(derive, register, reflect, init)]
#[derive(Default)]
struct FooRes3(usize);

#[auto_bind_plugin(plugin = Test)]
#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type]
#[auto_add_message]
struct FooEvent(usize);

#[auto_bind_plugin(plugin = Test)]
#[auto_event]
struct FooEvent2(usize);

#[auto_bind_plugin(plugin = Test)]
#[auto_event]
struct FooEvent3(usize);

#[auto_bind_plugin(plugin = Test)]
#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state]
#[auto_register_state_type]
enum FooState {
    #[default]
    Start,
    End,
}

#[auto_bind_plugin(plugin = Test)]
#[auto_add_system(schedule = Update)]
fn foo_system(mut foo_res: ResMut<FooRes>) {
    foo_res.0 += 1;
}

#[auto_bind_plugin(plugin = Test)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
struct FooComponentState {
    is_added: bool,
}

#[auto_bind_plugin(plugin = Test)]
#[auto_add_observer]
fn foo_observer(
    add: On<Add, FooComponent>,
    added_foo_q: Query<Ref<FooComponent>, Added<FooComponent>>,
    mut foo_component_added: ResMut<FooComponentState>,
) {
    assert!(
        added_foo_q
            .get(add.event().entity)
            .expect("FooComponent not spawned")
            .is_added()
    );
    foo_component_added.is_added = true;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_type_foo_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooComponent>()),
        "did not auto register type"
    );
}

#[internal_test_proc_macro::xtest]
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

#[internal_test_proc_macro::xtest]
fn test_auto_init_resource_foo_default_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooDefaultRes>(),
        Some(&FooDefaultRes::default()),
        "did not auto init resource"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource_foo_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(1)),
        "did not auto insert resource"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(1)),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(2)),
        "did not register system"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_message_foo_event() {
    let mut app = app();
    assert!(app.world_mut().write_message(FooEvent(1)).is_some());
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_state_type_foo_state() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<State<FooState>>()),
        "did not auto register type"
    );
    assert!(
        type_registry.contains(type_id_of::<NextState<FooState>>()),
        "did not auto register type"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_init_state_type_foo_state() {
    let app = app();
    assert_eq!(
        app.world()
            .get_resource::<State<FooState>>()
            .map(Deref::deref),
        Some(&FooState::Start),
        "did not auto init state"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_observer_foo_observer() {
    let mut app = app();
    assert!(
        !app.world()
            .get_resource::<FooComponentState>()
            .unwrap()
            .is_added,
        "FooComponent should not be added yet"
    );
    app.world_mut().spawn(FooComponent);
    assert!(
        app.world()
            .get_resource::<FooComponentState>()
            .unwrap()
            .is_added,
        "FooComponent should be added"
    );
}
