use bevy_app::prelude::*;
use bevy_asset::{
    AssetPlugin,
    Assets,
    prelude::*,
};
use bevy_auto_plugin::prelude::*;
use bevy_reflect::TypePath;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_asset(plugin = TestPlugin)]
#[derive(Asset, TypePath)]
struct TestAsset;

#[auto_init_asset(plugin = TestPlugin, generics(usize))]
#[derive(Asset, TypePath)]
struct GenericAsset<T: TypePath + Send + Sync + 'static>(T);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_init_asset() {
    let app = app();
    assert!(app.world().get_resource::<Assets<TestAsset>>().is_some(), "did not auto init asset");
}

#[xtest]
fn test_auto_init_asset_generic() {
    let app = app();
    assert!(
        app.world().get_resource::<Assets<GenericAsset<usize>>>().is_some(),
        "did not auto init generic asset"
    );
}
