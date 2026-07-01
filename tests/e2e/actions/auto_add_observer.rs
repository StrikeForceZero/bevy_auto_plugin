use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(Component)]
struct FooComponent;

#[derive(Resource, Debug, Default, PartialEq)]
#[auto_init_resource(plugin = TestPlugin)]
struct FooComponentState {
    is_added: bool,
    gated_add_count: usize,
}

#[derive(Resource, Debug, Default, PartialEq)]
#[auto_init_resource(plugin = TestPlugin)]
struct ObserverEnabled(bool);

fn observer_enabled(observer_enabled: Res<ObserverEnabled>) -> bool {
    observer_enabled.0
}

#[auto_add_observer(plugin = TestPlugin)]
fn test_observer(
    add: On<Add, FooComponent>,
    added_foo_q: Query<Ref<FooComponent>, Added<FooComponent>>,
    mut foo_component_added: ResMut<FooComponentState>,
) {
    assert!(added_foo_q.get(add.event().entity).expect("FooComponent not spawned").is_added());
    foo_component_added.is_added = true;
}

#[auto_add_observer(plugin = TestPlugin, config(run_if = observer_enabled, run_if = observer_enabled))]
fn gated_observer(_add: On<Add, FooComponent>, mut foo_component_added: ResMut<FooComponentState>) {
    foo_component_added.gated_add_count += 1;
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_add_observer() {
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

#[xtest]
fn test_auto_add_observer_run_if() {
    let mut app = app();
    app.world_mut().spawn(FooComponent);
    assert_eq!(
        app.world().get_resource::<FooComponentState>().unwrap().gated_add_count,
        0,
        "gated observer should not run before its condition is true"
    );

    app.world_mut().resource_mut::<ObserverEnabled>().0 = true;
    app.world_mut().spawn(FooComponent);
    assert_eq!(
        app.world().get_resource::<FooComponentState>().unwrap().gated_add_count,
        1,
        "gated observer should run after its condition is true"
    );
}
