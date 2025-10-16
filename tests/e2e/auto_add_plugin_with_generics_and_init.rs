use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(AutoPlugin)]
#[auto_plugin(impl_generic_auto_plugin_trait)]
#[auto_add_plugin(plugin = TestPlugin, generics(usize), init = TestSubPlugin(1))]
struct TestSubPlugin<T: 'static>(T);

impl<T: Send + Sync + Clone + 'static> Plugin for TestSubPlugin<T> {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.insert_resource(Test(self.0.clone()));
    }
}

#[derive(Resource, Debug, Default, PartialEq)]
struct Test<T>(T);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_add_plugin_with_generics_and_init() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test<usize>>(),
        Some(&Test(1)),
        "did not auto add plugin"
    );
}
