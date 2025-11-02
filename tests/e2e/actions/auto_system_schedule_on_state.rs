use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
struct TestPlugin;

impl Plugin for TestPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.init_resource::<Counter>();
        app.init_state::<TestState>();
    }
}

#[derive(Resource, Debug, Default, PartialEq)]
#[auto_init_resource(plugin = TestPlugin)]
struct Counter(usize);

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
enum TestState {
    #[default]
    Init,
    Run,
}

#[auto_system(plugin = TestPlugin, schedule = OnEnter(TestState::Run))]
fn system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_state::app::StatesPlugin;
    use internal_test_util::create_minimal_app;

    fn app() -> App {
        let mut app = create_minimal_app();
        app.add_plugins(StatesPlugin);
        app
    }

    #[xtest]
    fn test_system() {
        let mut app = app();
        app.add_plugins(TestPlugin);

        assert_eq!(app.world().get_resource::<Counter>().unwrap().0, 0);

        app.update();
        assert_eq!(app.world().get_resource::<Counter>().unwrap().0, 0);

        app.world_mut().resource_mut::<NextState<TestState>>().set(TestState::Run);
        app.update();

        assert_eq!(app.world().get_resource::<Counter>().unwrap().0, 1);
    }
}
