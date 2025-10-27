use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_type(plugin = TestPlugin, generics(1, 1))]
struct Foo<T1, T2>(T1, T2);

// dummy main
fn main() {}
