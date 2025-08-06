use bevy_app::prelude::*;
use bevy_auto_plugin::module::prelude::*;
use bevy_ecs::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;

    #[auto_init_resource]
    #[auto_insert_resource(resource(Test(1)))]
    #[derive(Resource, Default, Debug, PartialEq)]
    pub struct Test(pub usize);
}
use plugin_module::*;

fn plugin(app: &mut App) {
    plugin_module::init(app);
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test>(),
        Some(&Test(1)),
        "did not auto insert resource"
    );
}
