#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::app::StatesPlugin;
use internal_test_util::create_minimal_app;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

struct Foo;
struct Bar;

pub trait Thing {
    const AMOUNT: usize;
}

impl Thing for Foo {
    const AMOUNT: usize = 1;
}

impl Thing for Bar {
    const AMOUNT: usize = 2;
}

#[auto_resource(plugin = Test, derive(Debug, Default, Copy, Clone, PartialEq), init)]
struct FooResource(usize);

#[auto_run_on_build(plugin = Test, generics(Foo), generics(Bar))]
fn run_this<T: Thing + 'static>(app: &mut App) {
    let value = app
        .world_mut()
        .get_resource::<FooResource>()
        .copied()
        .unwrap_or_default()
        .0;
    app.world_mut()
        .insert_resource(FooResource(value + T::AMOUNT));
}

fn foo<T: Thing>(mut res: ResMut<FooResource>) {
    res.0 += T::AMOUNT;
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(Test);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_system_foo_system() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooResource>(),
        Some(&FooResource(3)),
        "did not run functions"
    );
}
