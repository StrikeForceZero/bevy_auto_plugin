use bevy::prelude::*;
use bevy_auto_plugin::global::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_util::{create_minimal_app, type_id_of};
use std::ops::Deref;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test)]
#[auto_name(plugin = Test)]
struct FooComponent;

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

#[derive(Event, Debug, Default, PartialEq, Reflect)]
#[auto_register_type(plugin = Test)]
#[auto_add_event(plugin = Test)]
struct FooEvent(usize);

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = Test)]
#[auto_register_state_type(plugin = Test)]
enum FooState {
    #[default]
    Start,
    End,
}

#[auto_add_system(plugin = Test, schedule = Update)]
fn foo_system(mut foo_res: ResMut<FooRes>) {
    foo_res.0 += 1;
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
fn test_auto_add_event_foo_event() {
    let mut app = app();
    assert!(app.world_mut().send_event(FooEvent(1)).is_some());
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
