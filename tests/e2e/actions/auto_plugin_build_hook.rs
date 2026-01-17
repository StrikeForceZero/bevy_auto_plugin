use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;
use std::{
    any::TypeId,
    collections::{
        HashMap,
        HashSet,
    },
};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(Resource, Debug, Default, PartialEq)]
struct Counter(HashMap<&'static str, HashMap<TypeId, HashSet<&'static str>>>);

struct MyCustomHookA;

impl<T: Component + 'static> AutoPluginBuildHook<T> for MyCustomHookA {
    fn on_build(&self, app: &mut App) {
        app.world_mut()
            .resource_mut::<Counter>()
            .0
            .entry("A")
            .or_default()
            .entry(TypeId::of::<T>())
            .or_default()
            .insert(std::any::type_name::<T>());
    }
}

struct MyCustomHookB(&'static str);

impl<T: Component + 'static> AutoPluginBuildHook<T> for MyCustomHookB {
    fn on_build(&self, app: &mut App) {
        let mut counter = app.world_mut().resource_mut::<Counter>();
        let set = counter.0.entry("B").or_default().entry(TypeId::of::<T>()).or_default();
        set.insert(std::any::type_name::<T>());
        set.insert(self.0);
    }
}

#[derive(Component, Debug)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = MyCustomHookA)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = MyCustomHookB("foo"))]
struct TestA;

#[derive(Component, Debug)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = MyCustomHookB("bar"))]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = MyCustomHookA)]
struct TestB;

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.init_resource::<Counter>();
    app.add_plugins(TestPlugin);
    app
}

#[xtest]
fn test_auto_plugin_hook() {
    let app = app();
    let counter = app.world().get_resource::<Counter>().expect("counter resource missing");

    let hook_a_map = counter.0.get("A").expect("Hook A entry missing");
    assert_eq!(
        hook_a_map.get(&TypeId::of::<TestA>()).expect("Hook A - TestA entry missing"),
        &HashSet::from([std::any::type_name::<TestA>()])
    );
    assert_eq!(
        hook_a_map.get(&TypeId::of::<TestB>()).expect("Hook A - TestB entry missing"),
        &HashSet::from([std::any::type_name::<TestB>()])
    );

    let hook_b_map = counter.0.get("B").expect("Hook B entry missing");
    assert_eq!(
        hook_b_map.get(&TypeId::of::<TestA>()).expect("Hook B - TestA entry missing"),
        &HashSet::from([std::any::type_name::<TestA>(), "foo"])
    );
    assert_eq!(
        hook_b_map.get(&TypeId::of::<TestB>()).expect("Hook B - TestB entry missing"),
        &HashSet::from([std::any::type_name::<TestB>(), "bar"])
    );
}
