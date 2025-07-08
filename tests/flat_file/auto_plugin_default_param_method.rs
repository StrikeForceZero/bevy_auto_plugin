use bevy_app::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use std::any::Any;

#[auto_register_type]
#[derive(Reflect)]
struct Test;

impl Test {
    #[auto_plugin]
    fn plugin(&self, my_app: &mut App) {}
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    Test.plugin(&mut app);
    app
}

#[test]
fn test_auto_plugin_param() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(Test.type_id()),
        "did not auto register type"
    );
}
