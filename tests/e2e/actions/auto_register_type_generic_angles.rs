use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::type_id_of;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_register_type(plugin = TestPlugin, generics = <bool, u32>, generics = <bool, bool>)]
#[auto_register_type(plugin = TestPlugin, generics = <usize, u8>)]
#[derive(Reflect)]
struct Test<A, B>(A, B);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_register_type_generic_angles() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<Test<bool, u32>>()),
        "did not auto register Test<bool, u32>"
    );
    assert!(
        type_registry.contains(type_id_of::<Test<bool, bool>>()),
        "did not auto register Test<bool, bool>"
    );
    assert!(
        type_registry.contains(type_id_of::<Test<usize, u8>>()),
        "did not auto register Test<usize, u8>"
    );
}
