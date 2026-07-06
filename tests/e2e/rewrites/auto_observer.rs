use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::create_minimal_app;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Component)]
struct Foo;

#[derive(Resource, Default)]
struct ObserverCount(usize);

#[derive(Resource, Default)]
struct ObserverEnabled(bool);

#[auto_observer(plugin = Test)]
fn foo_observer(_trigger: On<Add, Foo>, mut count: ResMut<ObserverCount>) {
    count.0 += 1;
}

fn observer_enabled(observer_enabled: Res<ObserverEnabled>) -> bool {
    observer_enabled.0
}

#[auto_observer(plugin = Test, config(run_if = observer_enabled, run_if = observer_enabled))]
fn gated_foo_observer(_trigger: On<Add, Foo>, mut count: ResMut<ObserverCount>) {
    count.0 += 10;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.init_resource::<ObserverCount>();
    app.init_resource::<ObserverEnabled>();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_observer() {
    let mut app = app();
    app.world_mut().spawn(Foo);
    assert_eq!(app.world().resource::<ObserverCount>().0, 1);
}

#[xtest]
fn test_auto_observer_run_if() {
    let mut app = app();
    app.world_mut().spawn(Foo);
    assert_eq!(app.world().resource::<ObserverCount>().0, 1);

    app.world_mut().resource_mut::<ObserverEnabled>().0 = true;
    app.world_mut().spawn(Foo);
    assert_eq!(app.world().resource::<ObserverCount>().0, 12);
}
