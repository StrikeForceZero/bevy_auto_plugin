use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_state::{
    app::StatesPlugin,
    prelude::*,
};
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_init_state(plugin = TestPlugin)]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

#[auto_init_sub_state(plugin = TestPlugin)]
#[derive(SubStates, Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]
enum IsPaused {
    #[default]
    Running,
    Paused,
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_init_state() {
    let app = app();
    assert!(
        app.world().get_resource::<State<AppState>>().is_some(),
        "did not auto init State<AppState>"
    );
    assert!(
        app.world().get_resource::<NextState<AppState>>().is_some(),
        "did not auto init NextState<AppState>"
    );
    assert!(
        app.world().get_resource::<State<IsPaused>>().is_none(),
        "State<IsPaused> shouldn't be set"
    );
    assert!(
        app.world().get_resource::<NextState<IsPaused>>().is_some(),
        "did not auto init NextState<IsPaused>"
    );
}

#[xtest]
fn test_update_state() {
    let mut app = app();

    app.update();

    assert_eq!(app.world().get_resource::<State<IsPaused>>().map(|state| state.get()), None,);

    app.world_mut().resource_mut::<NextState<AppState>>().into_inner().set(AppState::InGame);

    app.update();

    assert_eq!(app.world().resource::<State<IsPaused>>().get(), &IsPaused::Running,);

    app.world_mut().resource_mut::<NextState<IsPaused>>().into_inner().set(IsPaused::Paused);

    app.update();

    assert_eq!(app.world().resource::<State<IsPaused>>().get(), &IsPaused::Paused,);

    app.world_mut().resource_mut::<NextState<AppState>>().into_inner().set(AppState::Menu);

    app.update();

    assert_eq!(app.world().get_resource::<State<IsPaused>>().map(|state| state.get()), None,);
}
