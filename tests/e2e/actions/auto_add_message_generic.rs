use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[auto_add_message(plugin = TestPlugin, generics(bool))]
#[derive(Message, Debug, PartialEq)]
struct Test<T>(T);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_add_message_generic() {
    let mut app = app();
    let mut messages = app.world_mut().resource_mut::<Messages<Test<bool>>>();
    messages.write(Test(true));
    assert_eq!(messages.drain().next(), Some(Test(true)), "did not auto add event");
    assert_eq!(messages.drain().next(), None, "expected only 1 event");
}
