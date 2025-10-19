use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use internal_test_proc_macro::xtest;

mod a {
    pub mod b {
        pub mod c {
            use bevy::prelude::*;

            #[derive(Resource)]
            struct Counter(usize);
        }
    }
}

#[auto_init_resource(plugin = TestPlugin)]
use a::b::c::Counter;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_use_statement() {
    let app = app();
    assert!(
        app.world().get_resource::<Counter>().is_some(),
        "did not auto init resource"
    );
}
