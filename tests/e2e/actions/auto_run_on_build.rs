#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_util::create_minimal_app;
#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_run_on_build(plugin = Test)]
fn run_on_build(app: &mut App) {
    app.add_systems(Update, foo);
}

#[auto_resource(plugin = Test, derive(Debug, Default, PartialEq), init)]
struct FooResource(usize);

fn foo(mut res: ResMut<FooResource>) {
    res.0 += 1;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_system_foo_system() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooResource>(),
        Some(&FooResource(0)),
        "did not auto init resource"
    );
    app.update();
    assert_eq!(
        app.world().get_resource::<FooResource>(),
        Some(&FooResource(1)),
        "did not register system"
    );
}
