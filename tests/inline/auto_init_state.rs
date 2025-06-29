use bevy_app::prelude::*;
use bevy_auto_plugin::auto_plugin::*;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::*;

#[auto_init_state]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
enum Test {
    #[default]
    A,
    #[allow(dead_code)]
    B,
}

#[auto_init_state]
#[derive(SubStates, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[source(Test = Test::B)]
enum InnerTest {
    #[default]
    A,
}

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(plugin);
    app
}

#[test]
fn test_auto_init_state() {
    let app = app();
    assert!(
        app.world().get_resource::<State<Test>>().is_some(),
        "did not auto init state"
    );
    assert!(
        app.world().get_resource::<NextState<Test>>().is_some(),
        "did not auto init state"
    );
    assert!(
        app.world().get_resource::<State<InnerTest>>().is_some(),
        "did not auto init state"
    );
    assert!(
        app.world().get_resource::<NextState<InnerTest>>().is_some(),
        "did not auto init state"
    );
}
