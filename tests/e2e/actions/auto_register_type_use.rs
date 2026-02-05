use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;
use internal_test_util::type_id_of;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

mod external {
    use bevy_reflect::prelude::*;

    #[derive(Reflect)]
    pub struct ExternalA;

    #[derive(Reflect)]
    pub struct ExternalB;
}

#[auto_register_type(plugin = TestPlugin)]
use external::{
    ExternalA,
    ExternalB,
};

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_register_type_use() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(type_registry.contains(type_id_of::<ExternalA>()), "did not auto register ExternalA");
    assert!(type_registry.contains(type_id_of::<ExternalB>()), "did not auto register ExternalB");
}
