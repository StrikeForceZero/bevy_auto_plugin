use bevy_app::prelude::*;
use bevy_asset::{
    Asset,
    ReflectAsset,
};
use bevy_auto_plugin::prelude::*;
use bevy_ecs::reflect::AppTypeRegistry;
use bevy_reflect::{
    Reflect,
    TypePath,
};
use internal_test_proc_macro::xtest;
use std::any::TypeId;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_asset_reflect(plugin = TestPlugin)]
#[derive(Asset, Reflect)]
struct TestAsset;

#[auto_register_asset_reflect(plugin = TestPlugin, generics(usize))]
#[derive(Asset, Reflect)]
struct GenericAsset<T: Reflect + TypePath + Send + Sync + 'static>(T);

mod external {
    use bevy_asset::Asset;
    use bevy_reflect::Reflect;

    #[derive(Asset, Reflect)]
    pub struct ExternalAssetA;

    #[derive(Asset, Reflect)]
    pub struct ExternalAssetB;
}

#[auto_register_asset_reflect(plugin = TestPlugin)]
use external::{
    ExternalAssetA,
    ExternalAssetB,
};

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_register_asset_reflect() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.get_type_data::<ReflectAsset>(TypeId::of::<TestAsset>()).is_some(),
        "did not auto register asset reflection"
    );
}

#[xtest]
fn test_auto_register_asset_reflect_generic() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.get_type_data::<ReflectAsset>(TypeId::of::<GenericAsset<usize>>()).is_some(),
        "did not auto register generic asset reflection"
    );
}

#[xtest]
fn test_auto_register_asset_reflect_use() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.get_type_data::<ReflectAsset>(TypeId::of::<ExternalAssetA>()).is_some(),
        "did not auto register ExternalAssetA asset reflection"
    );
    assert!(
        type_registry.get_type_data::<ReflectAsset>(TypeId::of::<ExternalAssetB>()).is_some(),
        "did not auto register ExternalAssetB asset reflection"
    );
}
