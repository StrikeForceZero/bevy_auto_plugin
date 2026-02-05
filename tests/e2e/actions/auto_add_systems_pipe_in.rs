use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
struct TestPlugin;

impl Plugin for TestPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.init_resource::<Greeting>();
    }
}

#[derive(Resource, Debug, Default)]
struct Greeting(String);

fn get_name() -> String {
    "World".to_string()
}

fn greet_name(name: In<String>) -> String {
    format!("Hello, {}!", *name)
}

#[auto_add_system(plugin = TestPlugin, schedule = Update, pipe_in = [get_name, greet_name])]
fn set_greeting(greeting: In<String>, mut greeting_res: ResMut<Greeting>) {
    greeting_res.0 = greeting.0.clone();
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_add_systems_pipe_in() {
    let mut app = app();
    app.update();
    assert_eq!(app.world().resource::<Greeting>().0, "Hello, World!");
}
