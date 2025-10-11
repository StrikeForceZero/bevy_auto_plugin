#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::entity::EntityHashMap;
use bevy_state::app::StatesPlugin;
use internal_test_proc_macro::xtest;
use internal_test_util::{create_minimal_app, type_id_of, vec_spread};
use std::ops::Deref;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test)]
#[auto_name(plugin = Test)]
struct FooComponent;
#[auto_component(plugin = Test, derive, register, reflect, auto_name)]
struct FooComponent2;

#[auto_component(plugin = Test, derive, register, reflect, auto_name)]
struct FooComponent3;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test)]
#[auto_init_resource(plugin = Test)]
struct FooDefaultRes(usize);

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test)]
#[auto_init_resource(plugin = Test)]
#[auto_insert_resource(plugin = Test, resource(FooRes(1)))]
struct FooRes(usize);

#[auto_resource(plugin = Test, derive, register, reflect, init)]
#[derive(Default)]
struct FooRes2(usize);

#[auto_resource(plugin = Test, derive, register, reflect, init)]
#[derive(Default)]
struct FooRes3(usize);

#[auto_message(plugin = Test, derive)]
struct FooMessage(usize);

#[auto_event(plugin = Test, target(global), derive)]
struct FooGlobalEvent(usize);

#[auto_event(plugin = Test, target(entity), derive)]
struct FooEntityEvent {
    #[event_target]
    entity: Entity,
    value: usize,
}

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = Test)]
#[auto_register_state_type(plugin = Test)]
enum FooState {
    #[default]
    Start,
    End,
}

#[auto_states(plugin = Test, derive, register, reflect, init)]
enum FooState2 {
    #[default]
    Start,
    End,
}

#[auto_add_system(plugin = Test, schedule = Update)]
fn foo_system(mut foo_res: ResMut<FooRes>) {
    foo_res.0 += 1;
}

#[auto_system(plugin = Test, schedule = Update)]
fn foo_system2() {
    // TODO: add something
}

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test)]
#[auto_init_resource(plugin = Test)]
struct FooComponentState {
    is_added: bool,
}

#[auto_add_observer(plugin = Test)]
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

#[auto_observer(plugin = Test)]
fn foo_observer2(_add: On<Add, FooComponent>) {}

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
    assert!(
        type_registry.contains(type_id_of::<FooComponent>()),
        "did not auto register type"
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
    assert_eq!(
        name,
        &Name::new("FooComponent"),
        "did not auto name FooComponent"
    );
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
    assert_eq!(
        app.world().get_resource::<FooRes>(),
        Some(&FooRes(2)),
        "did not register system"
    );
}

#[xtest]
fn test_auto_add_message_foo_message() {
    let mut app = app();
    assert!(
        app.world_mut().write_message(FooMessage(1)).is_some(),
        "did not add message type FooMessage"
    );
}

#[xtest]
fn test_auto_event_foo_global_event() {
    let mut app = app();
    #[derive(Resource, Default)]
    struct Counter(usize);

    app.insert_resource(Counter(0));

    app.add_observer(|on: On<FooGlobalEvent>, mut counter: ResMut<Counter>| {
        counter.0 += 1;
        assert_eq!(counter.0, on.event().0);
    });

    app.world_mut().trigger(FooGlobalEvent(1));
    app.world_mut().trigger(FooGlobalEvent(2));

    assert_eq!(app.world_mut().resource::<Counter>().0, 2);
}

#[xtest]
fn test_auto_event_foo_entity_event() {
    let mut app = app();
    #[derive(Resource, Default)]
    struct Counter(EntityHashMap<usize>);

    app.insert_resource(Counter::default());

    fn entity_observer(on: On<FooEntityEvent>, mut counter: ResMut<Counter>) {
        let entry = counter.0.entry(on.entity).or_default();
        *entry += 1;
        assert_eq!(*entry, on.event().value);
    }

    let a = app.world_mut().spawn_empty().id();
    let b = app.world_mut().spawn_empty().id();
    let c = app.world_mut().spawn_empty().id();

    let entities = [a, b];

    for e in vec_spread![..entities, c] {
        app.world_mut().entity_mut(e).observe(entity_observer);
    }

    for e in entities {
        app.world_mut()
            .entity_mut(e)
            .trigger(|entity| FooEntityEvent { entity, value: 1 });
    }

    for e in entities {
        app.world_mut()
            .entity_mut(e)
            .trigger(|entity| FooEntityEvent { entity, value: 2 });
    }

    app.world_mut()
        .entity_mut(c)
        .trigger(|entity| FooEntityEvent { entity, value: 1 });

    for e in entities {
        let &v = app.world_mut().resource::<Counter>().0.get(&e).unwrap();
        assert_eq!(v, 2);
    }
}

#[xtest]
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

#[xtest]
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

#[xtest]
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
