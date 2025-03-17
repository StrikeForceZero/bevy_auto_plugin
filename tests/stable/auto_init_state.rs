use bevy_app::prelude::*;
use bevy_auto_plugin::auto_plugin_module::*;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;

    #[auto_init_state]
    #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub enum Test {
        #[default]
        A,
        B,
    }

    #[auto_init_state]
    #[derive(SubStates, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
    #[source(Test = Test::B)]
    pub enum InnerTest {
        #[default]
        A,
        B,
    }
}
use plugin_module::*;

fn plugin(app: &mut App) {
    plugin_module::init(app);
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(plugin);
    app
}

#[test]
fn test_auto_init_state() {
    let app = app();
    assert!(app.world().get_resource::<State<Test>>().is_some());
    assert!(app.world().get_resource::<NextState<Test>>().is_some());
    assert!(
        app.world().get_resource::<State<InnerTest>>().is_some(),
        "did not auto init state"
    );
    assert!(
        app.world().get_resource::<NextState<InnerTest>>().is_some(),
        "did not auto init state"
    );
}
