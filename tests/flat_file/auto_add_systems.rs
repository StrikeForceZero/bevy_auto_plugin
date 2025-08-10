use bevy_app::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource, Debug, Copy, Clone, Default, PartialEq)]
struct Test(i32);

#[auto_add_system(schedule = Update)]
fn foo_system(mut test: ResMut<Test>) {
    test.0 += 1;
}

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    app.init_resource::<Test>();
}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

fn test_eq(app: &App, b: i32) {
    assert_eq!(app.world().resource::<Test>(), &Test(b));
}

#[internal_test_proc_macro::xtest]
fn test_auto_register_systems() {
    let mut app = app();
    test_eq(&app, 0);
    app.update();
    test_eq(&app, 1);
    app.update();
    test_eq(&app, 2);
}
