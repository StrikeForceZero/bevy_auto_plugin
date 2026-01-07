use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_auto_plugin_proc_macros::{
    auto_init_resource,
    auto_insert_resource,
};
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
pub struct MyPlugin;

#[derive(Resource, Default, PartialEq, Debug)]
#[auto_init_resource(plugin = MyPlugin)]
#[auto_insert_resource(plugin = MyPlugin, init(MyResourceAuto(1)))]
pub struct MyResourceAuto(usize);

#[derive(Resource, Default, PartialEq, Debug)]
#[auto_init_resource(plugin = MyPlugin)]
pub struct MyResourceBuild(usize);

impl Plugin for MyPlugin {
    #[auto_plugin]
    fn build(&self, non_default_app_param_name: &mut App) {
        non_default_app_param_name.insert_resource(MyResourceBuild(1));
    }
}

#[xtest]
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
