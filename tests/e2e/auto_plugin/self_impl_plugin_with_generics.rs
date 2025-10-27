use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_auto_plugin_proc_macros::{
    auto_init_resource,
    auto_insert_resource,
};
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin, Default)]
#[auto_plugin]
pub struct MyPlugin<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Default, PartialEq, Debug)]
#[auto_init_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool))]
#[auto_insert_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool), resource(MyResourceAuto(1, true)))]
pub struct MyResourceAuto<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

#[derive(Resource, Default, PartialEq, Debug)]
#[auto_init_resource(plugin = MyPlugin::<u8, bool>, generics(u8, bool))]
pub struct MyResourceBuild<T1, T2>(T1, T2)
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static;

impl<T1, T2> Plugin for MyPlugin<T1, T2>
where
    T1: Default + Send + Sync + 'static,
    T2: Default + Send + Sync + 'static,
{
    #[auto_plugin]
    fn build(&self, non_default_app_param_name: &mut App) {
        non_default_app_param_name.insert_resource(MyResourceBuild(1u8, true));
    }
}

#[xtest]
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
