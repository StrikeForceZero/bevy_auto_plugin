use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[auto_add_message]
#[derive(Message)]
struct Test;

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_message() {
    let mut app = app();
    let mut messages = app.world_mut().resource_mut::<Messages<Test>>();
    messages.send(Test);
    assert_eq!(messages.drain().count(), 1, "did not auto add event");
}
