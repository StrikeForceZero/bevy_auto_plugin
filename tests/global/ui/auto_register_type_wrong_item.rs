use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_type(plugin = TestPlugin)]
#[derive(Reflect)]
fn bad_component() {}

// dummy main
fn main() {}
