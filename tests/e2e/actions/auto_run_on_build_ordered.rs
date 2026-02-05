#![allow(dead_code)]

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use internal_test_util::create_minimal_app;
#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct Test;

#[auto_resource(plugin = Test, derive(Debug, Default, PartialEq), init)]
struct FooResource(Vec<&'static str>);

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestRob;

macro_rules! rob {
    ($label:ident) => {
        #[auto_run_on_build(plugin = TestRob, post_build)]
        fn $label(app: &mut App) {
            app.world_mut()
                .run_system_once(insert(stringify!($label)))
                .expect("failed to run system");
        }
    };
}

rob!(a);
rob!(b);
rob!(c);
rob!(d);
rob!(e);

fn insert(label: &'static str) -> impl Fn(ResMut<FooResource>) {
    move |mut res: ResMut<FooResource>| {
        res.0.push(label);
    }
}

fn app() -> App {
    let mut app = create_minimal_app();
    app.add_plugins(Test);
    app
}

#[internal_test_proc_macro::xtest]
fn test_run_on_build_ordered() {
    let mut app = app();
    assert_eq!(
        app.world().get_resource::<FooResource>(),
        Some(&FooResource::default()),
        "did not auto init resource"
    );
    app.add_plugins(TestRob);
    assert!(!app.world().resource::<FooResource>().0.is_empty(), "did not run any systems");
    assert_eq!(
        app.world().resource::<FooResource>(),
        &FooResource(vec!["a", "b", "c", "d", "e"]),
        "did not run systems in order"
    );
}
