use bevy_app::prelude::*;
use bevy_auto_plugin::auto_plugin_module::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;

    #[auto_register_type]
    #[auto_register_state_type]
    #[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
    pub enum Test {
        #[default]
        A,
        B,
    }

    #[auto_register_type]
    #[auto_register_state_type]
    #[derive(SubStates, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
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
fn test_auto_register_state_type() {
    use std::any::Any;
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(State::new(Test::A).type_id()),
        "did not auto init state"
    );
    assert!(
        type_registry.contains(NextState::<Test>::Unchanged.type_id()),
        "did not auto init state"
    );
    assert!(
        type_registry.contains(NextState::<InnerTest>::Unchanged.type_id()),
        "did not auto register state type"
    );
    assert!(
        type_registry.contains(State::new(InnerTest::A).type_id()),
        "did not auto register state type"
    );
}
