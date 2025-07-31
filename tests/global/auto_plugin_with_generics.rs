use bevy_app::prelude::*;
use bevy_auto_plugin::global::prelude::{AutoPlugin, auto_register_type};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use internal_test_util::{create_minimal_app, type_id_of};
use std::any::Any;

#[derive(AutoPlugin)]
#[auto_plugin(
    generics(u32, i32),
    impl_generic_plugin_trait,
    impl_generic_auto_plugin_trait
)]
struct Test<T1, T2>(T1, T2)
where
    T1: Send + Sync + 'static,
    T2: Send + Sync + 'static;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = Test::<u32, i32>, generics(u32, i32))]
struct Foo<T1, T2>(T1, T2)
where
    T1: Any + Send + Sync + 'static,
    T2: Any + Send + Sync + 'static;

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(bevy_auto_plugin_shared::global::__internal::bevy_log::LogPlugin::default());
    app.add_plugins(Test(0u32, 0i32));
    app
}

#[test]
fn test_auto_register_type() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<Foo::<u32, i32>>()),
        "did not auto register type"
    );
}
