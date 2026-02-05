#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_proc_macro::xtest;
use internal_test_util::{
    create_minimal_app,
    type_id_of,
};
use std::ops::Deref;

#[derive(AutoPlugin)]
struct Test;

impl Plugin for Test {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        assert_end_type_registry(app, false);
    }
}

fn assert_end_type_registry(app: &App, expected: bool) {
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    let entries = [
        ("FooComponent", type_id_of::<FooComponent>()),
        ("FooComponent2", type_id_of::<FooComponent2>()),
        ("FooComponent3", type_id_of::<FooComponent3>()),
        ("FooDefaultRes", type_id_of::<FooDefaultRes>()),
        ("FooRes", type_id_of::<FooRes>()),
        ("FooRes2", type_id_of::<FooRes2>()),
        ("FooRes3", type_id_of::<FooRes3>()),
        ("FooEvent", type_id_of::<FooEvent>()),
        ("FooComponentState", type_id_of::<FooComponentState>()),
        ("State<FooState>", type_id_of::<State<FooState>>()),
        ("NextState<FooState>", type_id_of::<NextState<FooState>>()),
    ];
    let timing = if expected { "after" } else { "before" };
    for (name, type_id) in entries {
        assert_eq!(
            type_registry.contains(type_id),
            expected,
            "{name} should be registered {timing} end bindings run"
        );
    }
}

#[auto_bind_plugin(plugin = Test, end)]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type]
#[auto_name]
struct FooComponent;

#[auto_bind_plugin(plugin = Test, end)]
#[auto_component(derive, register, reflect, auto_name)]
struct FooComponent2;

#[auto_bind_plugin(plugin = Test, end)]
#[auto_component(derive, register, reflect, auto_name)]
struct FooComponent3;

#[auto_bind_plugin(plugin = Test, end)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
struct FooDefaultRes(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
#[auto_insert_resource(insert(FooRes(1)))]
struct FooRes(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[auto_resource(derive, register, reflect, init)]
#[derive(Default)]
struct FooRes2(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[auto_resource(derive, register, reflect, init)]
#[derive(Default)]
struct FooRes3(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[derive(Message, Debug, Default, PartialEq, Reflect)]
#[auto_register_type]
#[auto_add_message]
struct FooEvent(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[auto_event]
struct FooEvent2(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[auto_event]
struct FooEvent3(usize);

#[auto_bind_plugin(plugin = Test, end)]
#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state]
#[auto_register_state_type]
enum FooState {
    #[default]
    Start,
    End,
}

#[auto_bind_plugin(plugin = Test, end)]
#[auto_add_system(schedule = Update)]
fn foo_system(mut foo_res: ResMut<FooRes>) {
    foo_res.0 += 1;
}

#[auto_bind_plugin(plugin = Test, end)]
#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type]
#[auto_init_resource]
struct FooComponentState {
    is_added: bool,
}

#[auto_bind_plugin(plugin = Test, end)]
#[auto_add_observer]
fn foo_observer(
    add: On<Add, FooComponent>,
    added_foo_q: Query<Ref<FooComponent>, Added<FooComponent>>,
    mut foo_component_added: ResMut<FooComponentState>,
) {
    assert!(added_foo_q.get(add.event().entity).expect("FooComponent not spawned").is_added());
    foo_component_added.is_added = true;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_bind_plugin_end_order() {
    let app = app();
    assert_end_type_registry(&app, true);
}

#[xtest]
fn test_auto_register_type_foo_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<FooComponent>()), "did not auto register type");
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
fn test_auto_init_resource_foo_default_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooDefaultRes>(),
        Some(&FooDefaultRes::default()),
        "did not auto init resource"
    );
}

#[xtest]
fn test_auto_insert_resource_foo_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(1)),
        "did not auto insert resource"
    );
}

#[xtest]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(1)),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(app.world().get_resource::<FooRes>(), Some(&FooRes(2)), "did not register system");
}

#[xtest]
fn test_auto_add_message_foo_event() {
    let mut app = app();
    assert!(app.world_mut().write_message(FooEvent(1)).is_some());
}

#[xtest]
fn test_auto_register_state_type_foo_state() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<State<FooState>>()), "did not auto register type");
    assert!(
        type_registry.contains(type_id_of::<NextState<FooState>>()),
        "did not auto register type"
    );
}

#[xtest]
fn test_auto_init_state_type_foo_state() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<State<FooState>>().map(Deref::deref),
        Some(&FooState::Start),
        "did not auto init state"
    );
}

#[xtest]
fn test_auto_add_observer_foo_observer() {
    let mut app = app();
    assert!(
        !app.world().get_resource::<FooComponentState>().unwrap().is_added,
        "FooComponent should not be added yet"
    );
    app.world_mut().spawn(FooComponent);
    assert!(
        app.world().get_resource::<FooComponentState>().unwrap().is_added,
        "FooComponent should be added"
    );
}
