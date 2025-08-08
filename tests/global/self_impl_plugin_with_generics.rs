use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;
use bevy_auto_plugin_proc_macros::{global_auto_init_resource, global_auto_insert_resource};

#[derive(AutoPlugin, Default)]
#[auto_plugin(generics(u8, bool))]
pub struct MyPlugin<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Default, PartialEq, Debug)]
#[global_auto_init_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool))]
#[global_auto_insert_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool), resource(MyResourceAuto(1, true)))]
pub struct MyResourceAuto<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Default, PartialEq, Debug)]
#[global_auto_init_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool))]
pub struct MyResourceBuild<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

impl Plugin for MyPlugin<u8, bool> {
    #[global_auto_plugin(app_param=non_default_app_param_name)]
    fn build(&self, non_default_app_param_name: &mut App) {
        non_default_app_param_name.insert_resource(MyResourceBuild(1u8, true));
    }
}

#[internal_test_proc_macro::xtest]
fn test() {
    let mut app = App::new();
    app.add_plugins(MyPlugin::<u8, bool>::default());
    assert_eq!(
        app.world().get_resource::<MyResourceBuild::<u8, bool>>(),
        Some(&MyResourceBuild(1u8, true)),
        "build function was not called"
    );
    assert_eq!(
        app.world().get_resource::<MyResourceAuto::<u8, bool>>(),
        Some(&MyResourceAuto(1u8, true)),
        "auto plugin failed"
    );
}
