use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Component)]
struct FooComponent;

#[derive(Resource, Debug, Default, PartialEq)]
#[auto_init_resource]
struct FooComponentState {
    is_added: bool,
}

#[auto_add_observer]
fn test_observer(
    add: On<Add, FooComponent>,
    added_foo_q: Query<Ref<FooComponent>, Added<FooComponent>>,
    mut foo_component_added: ResMut<FooComponentState>,
) {
    assert!(
        added_foo_q
            .get(add.event().entity)
            .expect("FooComponent not spawned")
            .is_added()
    );
    foo_component_added.is_added = true;
}

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_observer() {
    let mut app = app();
    assert!(
        !app.world()
            .get_resource::<FooComponentState>()
            .unwrap()
            .is_added,
        "FooComponent should not be added yet"
    );
    app.world_mut().spawn(FooComponent);
    assert!(
        app.world()
            .get_resource::<FooComponentState>()
            .unwrap()
            .is_added,
        "FooComponent should be added"
    );
}
