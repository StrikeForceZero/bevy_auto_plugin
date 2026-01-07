use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::create_minimal_app;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[derive(Resource, Default)]
struct SystemCounter(Vec<&'static str>);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum TestSet {
    Set,
}

#[auto_system(plugin = Test, schedule = Update)]
fn basic_system(mut counter: ResMut<SystemCounter>) {
    counter.0.push("basic");
}

#[auto_system(plugin = Test, schedule = Update, config(in_set = TestSet::Set))]
fn set_system(mut counter: ResMut<SystemCounter>) {
    counter.0.push("set");
}

#[auto_system(plugin = Test, schedule = Update, config(after = basic_system))]
fn after_system(mut counter: ResMut<SystemCounter>) {
    counter.0.push("after");
}

#[auto_system(plugin = Test, schedule = Update, generics(usize))]
fn generic_system<T: Send + Sync + 'static>(mut counter: ResMut<SystemCounter>) {
    counter.0.push("generic");
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.init_resource::<SystemCounter>();
    app.configure_sets(Update, TestSet::Set);
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_systems() {
    let mut app = app();
    app.update();
    let counter = &app.world().resource::<SystemCounter>().0;
    assert!(counter.contains(&"basic"), "did not auto register basic system");
    assert!(counter.contains(&"set"), "did not auto register set system");
    assert!(counter.contains(&"after"), "did not auto register after system");
    assert!(counter.contains(&"generic"), "did not auto register generic system");
    assert_eq!(counter.len(), 4);
    assert!(
        counter.iter().position(|s| s == &"after").unwrap()
            > counter.iter().position(|s| s == &"basic").unwrap(),
        "after system not executed after basic system"
    );
}
