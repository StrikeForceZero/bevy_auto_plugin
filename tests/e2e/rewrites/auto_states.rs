use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_proc_macro::xtest;
use internal_test_util::type_id_of;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_states(plugin = Test, derive, reflect, register, init)]
enum FooState {
    #[default]
    Start,
    End,
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_init_state() {
    let app = app();
    assert_eq!(app.world().resource::<State<FooState>>().get(), &FooState::Start);
}

#[xtest]
fn test_auto_register_types() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<FooState>()), "did not auto register type");
    assert!(
        type_registry.contains(type_id_of::<State<FooState>>()),
        "did not auto register State type"
    );
    assert!(
        type_registry.contains(type_id_of::<NextState<FooState>>()),
        "did not auto register NextState type"
    );
}
