use bevy_app::prelude::*;
use bevy_asset::{
    Asset,
    AssetPlugin,
    Assets,
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
struct Test;

#[auto_asset(plugin = Test, derive(Debug, Default), init)]
struct FooAsset;

#[auto_asset(plugin = Test, generics(usize), derive(Debug, Default), init)]
struct GenericAsset<T: TypePath + Send + Sync + 'static>(T);

#[auto_asset(plugin = Test, init)]
#[derive(Asset, TypePath)]
struct ManuallyDerivedAsset;

#[auto_asset(plugin = Test, derive(Debug), reflect(Debug), register, init)]
struct ReflectedAsset {
    value: usize,
}

#[auto_asset(plugin = Test, generics(usize), derive, reflect, register)]
struct GenericReflectedAsset<T: Reflect + TypePath + Send + Sync + 'static>(T);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(Test);
    app
}

#[xtest]
fn test_auto_asset_derives_type_path() {
    assert!(FooAsset::type_path().contains("FooAsset"));
}

#[xtest]
fn test_auto_init_asset() {
    let app = app();
    assert!(app.world().contains_resource::<Assets<FooAsset>>());
}

#[xtest]
fn test_auto_init_generic_asset() {
    let app = app();
    assert!(app.world().contains_resource::<Assets<GenericAsset<usize>>>());
}

#[xtest]
fn test_auto_asset_can_init_manually_derived_asset() {
    let app = app();
    assert!(app.world().contains_resource::<Assets<ManuallyDerivedAsset>>());
}

#[xtest]
fn test_auto_asset_reflect_derives_reflect() {
    fn assert_reflect<T: Reflect>() {}

    assert_reflect::<ReflectedAsset>();
    assert!(ReflectedAsset::type_path().contains("ReflectedAsset"));
}

#[xtest]
fn test_auto_init_reflected_asset() {
    let app = app();
    assert!(app.world().contains_resource::<Assets<ReflectedAsset>>());
}

#[xtest]
fn test_auto_asset_registers_asset_reflect() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.get_type_data::<ReflectAsset>(TypeId::of::<ReflectedAsset>()).is_some());
}

#[xtest]
fn test_auto_asset_registers_generic_asset_reflect() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry
            .get_type_data::<ReflectAsset>(TypeId::of::<GenericReflectedAsset<usize>>())
            .is_some()
    );
}
