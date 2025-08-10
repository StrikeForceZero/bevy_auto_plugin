use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_state::app::StatesPlugin;
use bevy_state::prelude::*;
use internal_test_util::type_id_of;

#[auto_register_type]
#[auto_register_state_type]
#[derive(States, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
enum Test {
    #[default]
    A,
    B,
}

#[auto_register_type]
#[auto_register_state_type]
#[derive(SubStates, Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Reflect)]
#[source(Test = Test::B)]
enum InnerTest {
    #[default]
    A,
    B,
}

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    app.init_state::<Test>();
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(StatesPlugin);
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_state_type() {
    let app = app();
    let type_registry = app.world().resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    assert!(
        type_registry.contains(type_id_of::<NextState<Test>>()),
        "did not auto register state type"
    );
    assert!(
        type_registry.contains(type_id_of::<State<Test>>()),
        "did not auto register state type"
    );
    assert!(
        type_registry.contains(type_id_of::<NextState<InnerTest>>()),
        "did not auto register state type"
    );
    assert!(
        type_registry.contains(type_id_of::<State<InnerTest>>()),
        "did not auto register state type"
    );
}
