use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_auto_plugin_proc_macros::{global_auto_init_resource, global_auto_insert_resource};

#[derive(AutoPlugin)]
pub struct MyPlugin;

#[derive(Resource, Default, PartialEq, Debug)]
#[global_auto_init_resource(plugin = MyPlugin)]
#[global_auto_insert_resource(plugin = MyPlugin, resource(MyResourceAuto(1)))]
pub struct MyResourceAuto(usize);

#[derive(Resource, Default, PartialEq, Debug)]
#[global_auto_init_resource(plugin = MyPlugin)]
pub struct MyResourceBuild(usize);

#[auto_plugin(plugin = MyPlugin)]
fn build(app: &mut App) {
    app.insert_resource(MyResourceBuild(1));
}

#[internal_test_proc_macro::xtest]
fn test() {
    let mut app = App::new();
    app.add_plugins(MyPlugin);
    assert_eq!(
        app.world().get_resource::<MyResourceBuild>(),
        Some(&MyResourceBuild(1)),
        "build function was not called"
    );
    assert_eq!(
        app.world().get_resource::<MyResourceAuto>(),
        Some(&MyResourceAuto(1)),
        "auto plugin failed"
    );
}
