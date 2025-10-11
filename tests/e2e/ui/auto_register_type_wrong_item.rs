use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_type(plugin = TestPlugin)]
fn bad_component() {}

// dummy main
fn main() {}
