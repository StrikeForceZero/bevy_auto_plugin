use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[auto_add_message(generics(bool))]
#[derive(Message, Debug, PartialEq)]
struct Test<T>(T);

#[auto_plugin(app_param=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_message_generic() {
    let mut app = app();
    let mut messages = app.world_mut().resource_mut::<Messages<Test<bool>>>();
    messages.send(Test(true));
    assert_eq!(
        messages.drain().next(),
        Some(Test(true)),
        "did not auto add event"
    );
    assert_eq!(messages.drain().next(), None, "expected only 1 event");
}
