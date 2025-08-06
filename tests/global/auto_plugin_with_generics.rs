use bevy::prelude::*;
use bevy_auto_plugin::global::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_util::{create_minimal_app, type_id_of};
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
struct FooRes<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Event, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = Test::<u8, bool>, generics(u8, bool))]
#[auto_add_event(plugin = Test::<u8, bool>, generics(u8, bool))]
struct FooEvent<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

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
        &Name::new("FooComponent<u8,bool>"),
        "did not auto name FooComponent"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_init_resource_foo_res() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes::<u8, bool>::default()),
        "did not auto init resource"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes::<u8, bool>::default()),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(
        app.world().get_resource::<FooRes<u8, bool>>(),
        Some(&FooRes(1u8, false)),
        "did not register system"
    );
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_event_foo_event() {
    let mut app = app();
    assert!(app.world_mut().send_event(FooEvent(1u8, false)).is_some());
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
