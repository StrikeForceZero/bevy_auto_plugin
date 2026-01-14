use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::type_id_of;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_type(plugin = TestPlugin, generics(T2 = bool, T1 = u32))]
#[derive(Reflect)]
struct Test<T1, T2>(T1, T2);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_register_type_named_generics() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<Test<u32, bool>>()), "did not auto register type",);
}
