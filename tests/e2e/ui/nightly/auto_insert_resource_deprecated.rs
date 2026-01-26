#![deny(warnings)]

use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_insert_resource(plugin = TestPlugin, init(LegacyInit(1)))]
#[derive(Resource, Debug, PartialEq)]
struct LegacyInit(usize);

#[auto_insert_resource(plugin = TestPlugin, resource(LegacyResource(2)))]
#[derive(Resource, Debug, PartialEq)]
struct LegacyResource(usize);

fn main() {}
