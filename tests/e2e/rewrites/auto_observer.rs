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

#[auto_observer(plugin = Test)]
fn foo_observer(_trigger: On<Add, Foo>, mut count: ResMut<ObserverCount>) {
    count.0 += 1;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.init_resource::<ObserverCount>();
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_observer() {
    let mut app = app();
    app.world_mut().spawn(Foo);
    assert_eq!(app.world().resource::<ObserverCount>().0, 1);
}
