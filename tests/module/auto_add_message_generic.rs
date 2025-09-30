use bevy_app::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;
use bevy_ecs::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;
    #[auto_add_message(generics(bool))]
    #[derive(Message, Debug, PartialEq)]
    pub struct Test<T>(pub T);
}
use plugin_module::*;

fn plugin(app: &mut App) {
    plugin_module::init(app);
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_add_message_generic() {
    let mut app = app();
    let mut messages = app.world_mut().resource_mut::<Messages<Test<bool>>>();
    messages.write(Test(true));
    assert_eq!(
        messages.drain().next(),
        Some(Test(true)),
        "did not auto add event"
    );
    assert_eq!(messages.drain().next(), None, "expected only 1 event");
}
