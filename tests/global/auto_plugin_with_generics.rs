use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_ecs::entity::EntityHashMap;
use bevy_state::app::StatesPlugin;
use internal_test_util::{create_minimal_app, type_id_of, vec_spread};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Deref};

#[derive(AutoPlugin, Default)]
#[auto_plugin(impl_generic_plugin_trait, impl_generic_auto_plugin_trait)]
struct Test<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_name(plugin = Test::<u8, bool>, generics(u8, bool))]
struct FooComponent<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_init_resource(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_insert_resource(plugin = Test::<u8, bool>, generics(u8, bool), resource(FooRes(1, true)))]
struct FooRes<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_init_resource(plugin = Test::<u8, bool>, generics(u8, bool))]
struct FooDefaultRes<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[auto_message(plugin = Test::<u8, bool>, derive, generics(u8, bool))]
struct FooMessage<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[auto_event(plugin = Test::<u8, bool>, global, derive, generics(u8, bool))]
struct FooGlobalEvent<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[auto_event(plugin = Test, entity, derive)]
struct FooEntityEvent<T1, T2>
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static,
{
    #[event_target]
    entity: Entity,
    value1: T1,
    #[allow(dead_code)]
    value2: T2,
}

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_register_state_type(plugin = Test::<u8, bool>, generics(u8, bool))]
enum FooState<T1, T2>
where
    T1: Default + Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
    T2: Default + Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    Start(T1, T2),
    End(T1, T2),
}

impl<T1, T2> Default for FooState<T1, T2>
where
    T1: Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
    T2: Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::Start(T1::default(), T2::default())
    }
}

#[derive(Resource, Debug, Default, PartialEq, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = Test::<u8, bool>)]
#[auto_init_resource(plugin = Test::<u8, bool>)]
struct FooComponentState {
    is_added: bool,
}

#[allow(clippy::type_complexity)]
#[auto_add_observer(plugin = Test::<u8, bool>, generics(u8, bool))]
fn foo_observer<T1, T2>(
    add: On<Add, FooComponent<T1, T2>>,
    added_foo_q: Query<Ref<FooComponent<T1, T2>>, Added<FooComponent<T1, T2>>>,
    mut foo_component_added: ResMut<FooComponentState>,
) where
    T1: Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
    T2: Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    assert!(
        added_foo_q
            .get(add.event().entity)
            .expect("FooComponent not spawned")
            .is_added()
    );
    foo_component_added.is_added = true;
}

trait One {
    fn one() -> Self;
}

impl One for u8 {
    fn one() -> Self {
        1
    }
}

#[auto_add_system(plugin = Test::<u8, bool>, generics(u8, bool), schedule = Update)]
fn foo_system<T1, T2>(mut foo_res: ResMut<FooRes<T1, T2>>)
where
    T1: AddAssign
        + One
        + Debug
        + Default
        + Copy
        + Clone
        + PartialEq
        + Eq
        + Hash
        + Send
        + Sync
        + 'static,
    T2: Debug + Default + Copy + Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    foo_res.0 += T1::one();
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test::<u8, bool>::default());
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_type_foo_component() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<FooComponent<u8, bool>>()),
        "did not auto register type"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_name_foo_component() {
    let mut app = app();
    app.world_mut().spawn(FooComponent::<u8, bool>::default());
    let name = app
        .world_mut()
        .query_filtered::<&Name, With<FooComponent<u8, bool>>>()
        .single(app.world())
        .expect("failed to query FooComponent");
    assert_eq!(
        name,
        &Name::new("FooComponent<u8, bool>"),
        "did not auto name FooComponent"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_init_resource_foo_default_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooDefaultRes<u8, bool>>(),
        Some(&FooDefaultRes::<u8, bool>::default()),
        "did not auto init resource"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource_foo_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes(1, true)),
        "did not auto init resource"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes(1, true)),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes(2, true)),
        "did not register system"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_message_foo_event() {
    let mut app = app();
    assert!(
        app.world_mut()
            .write_message(FooMessage(1u8, false))
            .is_some(),
        "did not add message type FooMessage"
    );
}
#[should_panic]
/// TODO: `Event` doesn't support generics
#[internal_test_proc_macro::xtest]
fn test_auto_event_foo_global_event() {
    let mut app = app();
    #[derive(Resource, Default)]
    struct Counter(u8);

    app.insert_resource(Counter(0));

    app.add_observer(
        |on: On<FooGlobalEvent<u8, bool>>, mut counter: ResMut<Counter>| {
            counter.0 += 1;
            assert_eq!(counter.0, on.event().0);
        },
    );

    app.world_mut().trigger(FooGlobalEvent(1, false));
    app.world_mut().trigger(FooGlobalEvent(2, false));

    assert_eq!(app.world_mut().resource::<Counter>().0, 2);
}

#[should_panic]
/// TODO: `EntityEvent` doesn't support generics
#[internal_test_proc_macro::xtest]
fn test_auto_event_foo_entity_event() {
    let mut app = app();
    #[derive(Resource, Default)]
    struct Counter(EntityHashMap<u8>);

    app.insert_resource(Counter::default());

    fn entity_observer(on: On<FooEntityEvent<u8, bool>>, mut counter: ResMut<Counter>) {
        let entry = counter.0.entry(on.entity).or_default();
        *entry += 1;
        assert_eq!(*entry, on.event().value1);
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
            .trigger(|entity| FooEntityEvent {
                entity,
                value1: 1,
                value2: false,
            });
    }

    for e in entities {
        app.world_mut()
            .entity_mut(e)
            .trigger(|entity| FooEntityEvent {
                entity,
                value1: 2,
                value2: false,
            });
    }

    app.world_mut()
        .entity_mut(c)
        .trigger(|entity| FooEntityEvent {
            entity,
            value1: 1,
            value2: false,
        });

    for e in entities {
        let &v = app.world_mut().resource::<Counter>().0.get(&e).unwrap();
        assert_eq!(v, 2);
    }
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_state_type_foo_state() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<State<FooState<u8, bool>>>()),
        "did not auto register type"
    );
    assert!(
        type_registry.contains(type_id_of::<NextState<FooState<u8, bool>>>()),
        "did not auto register type"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_init_state_type_foo_state() {
    let app = app();
    assert_eq!(
        app.world()
            .get_resource::<State<FooState<u8, bool>>>()
            .map(Deref::deref),
        Some(&FooState::<u8, bool>::default()),
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
    app.world_mut().spawn(FooComponent::<u8, bool>::default());
    assert!(
        app.world()
            .get_resource::<FooComponentState>()
            .unwrap()
            .is_added,
        "FooComponent should be added"
    );
}
