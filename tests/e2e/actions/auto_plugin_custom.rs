use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_auto_plugin_shared::__private::auto_plugin_registry::AutoPluginCustom;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;
use std::any::TypeId;
use syn::{
    Path,
    parse_quote,
};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(Resource, Debug, Default, PartialEq)]
struct Counter(Vec<String>);

struct MyCustomHookA;

impl AutoPluginCustom for MyCustomHookA {
    fn resolve_path() -> Path {
        parse_quote!(::tests::e2e::actions::auto_plugin_custom::MyCustomHookA)
    }
    fn on_build<T: 'static>(app: &mut App) {
        app.world_mut().resource_mut::<Counter>().0.push(format!("A {:?}", TypeId::of::<T>()))
    }
}

struct MyCustomHookB;

impl AutoPluginCustom for MyCustomHookB {
    fn resolve_path() -> Path {
        parse_quote!(::tests::e2e::actions::auto_plugin_custom::MyCustomHookB)
    }
    fn on_build<T: 'static>(app: &mut App) {
        app.world_mut().resource_mut::<Counter>().0.push(format!("B {:?}", TypeId::of::<T>()))
    }
}

#[derive(Debug)]
#[auto_plugin_custom(plugin = TestPlugin, custom = MyCustomHookA)]
#[auto_plugin_custom(plugin = TestPlugin, custom = MyCustomHookB)]
struct TestA;

#[derive(Debug)]
#[auto_plugin_custom(plugin = TestPlugin, custom = MyCustomHookB)]
#[auto_plugin_custom(plugin = TestPlugin, custom = MyCustomHookA)]
struct TestB;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.init_resource::<Counter>();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_plugin_custom() {
    let app = app();
    fn hook_a(test: TypeId) -> String {
        format!("A {:?}", test)
    }
    fn hook_b(test: TypeId) -> String {
        format!("B {:?}", test)
    }
    let test_a = TypeId::of::<TestA>();
    let test_b = TypeId::of::<TestB>();
    assert_eq!(
        app.world().get_resource::<Counter>(),
        Some(&Counter(vec![
            hook_a(test_a),
            hook_a(test_b),
            hook_b(test_b),
            hook_b(test_a),
        ]))
    );
}
