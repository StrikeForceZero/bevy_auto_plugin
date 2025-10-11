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

#[auto_resource(plugin = Test, derive(Debug, Default, PartialEq), init)]
struct FooResource(usize);

#[auto_run_on_build(plugin = Test, generics(Foo), generics(Bar))]
fn run_this<T: Thing>(app: &mut App) {
    let stuff = T::AMOUNT;
    app.world_mut().resource_mut::<FooResource>().0 += stuff;
}

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
    let app = app();
    assert_eq!(
        app.world().get_resource::<FooResource>(),
        Some(&FooResource(3)),
        "did not run functions"
    );
}
