use bevy::prelude::*;
use bevy_auto_plugin::modes::global::prelude::*;

#[derive(AutoPlugin)]
struct TestPlugin;

impl Plugin for TestPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.init_resource::<Counter>();
    }
}

#[derive(Resource, Debug, Default, PartialEq)]
#[auto_init_resource(plugin = TestPlugin)]
struct Counter {
    count: usize,
    ran_last: Option<&'static str>,
}

impl Counter {
    fn get(&self) -> (usize, Option<&'static str>) {
        (self.count, self.ran_last)
    }
    fn increment(&mut self, label: &'static str) -> Option<&'static str> {
        self.count += 1;
        self.ran_last.replace(label)
    }
}

fn call_test<T>(s: T) -> T {
    s
}

macro_rules! macro_test {
    ($sys:ident) => {
        $sys
    };
}

#[auto_system(plugin = TestPlugin, schedule = Update, config(before = system_c, after = system_a))]
fn system_b(mut counter: ResMut<Counter>) {
    assert_eq!(counter.increment("system_b"), Some("system_a"));
}

#[auto_system(plugin = TestPlugin, schedule = Update, config(after = system_a, after = system_b))]
fn system_c(mut counter: ResMut<Counter>) {
    assert_eq!(counter.increment("system_c"), Some("system_b"));
}

#[auto_system(plugin = TestPlugin, schedule = Update, config(before = call_test(system_b), before = macro_test!(system_c)))]
fn system_a(mut counter: ResMut<Counter>) {
    if counter.count == 0 {
        assert_eq!(counter.increment("system_a"), None);
    } else {
        assert_eq!(counter.increment("system_c"), Some("system_c"));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_state::app::StatesPlugin;
    use internal_test_util::create_minimal_app;

    fn app() -> App {
        let mut app = create_minimal_app();
        app.add_plugins(StatesPlugin);
        app
    }

    #[internal_test_proc_macro::xtest]
    fn test_system() {
        let mut app = app();
        app.add_plugins(TestPlugin);

        assert_eq!(
            app.world().get_resource::<Counter>().unwrap().get(),
            (0, None)
        );

        app.update();

        assert_eq!(
            app.world().get_resource::<Counter>().unwrap().get(),
            (3, Some("system_c"))
        );
    }
}
