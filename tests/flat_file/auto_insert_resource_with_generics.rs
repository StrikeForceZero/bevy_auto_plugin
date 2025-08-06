use bevy_app::prelude::*;
use bevy_auto_plugin::flat_file::prelude::*;
use bevy_ecs::prelude::*;

#[auto_init_resource(generics(usize, bool))]
#[auto_insert_resource(generics(usize, bool), resource(Test(1, true)))]
#[derive(Resource, Debug, Default, PartialEq)]
struct Test<T1, T2>(T1, T2);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(plugin);
    app
}

#[internal_test_proc_macro::xtest]
fn test_auto_insert_resource() {
    let app = app();
    assert_eq!(
        app.world().get_resource::<Test::<usize, bool>>(),
        Some(&Test(1, true)),
        "did not auto insert resource"
    );
}
