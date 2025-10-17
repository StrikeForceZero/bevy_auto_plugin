use bevy_app::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs::prelude::*;
use internal_test_proc_macro::xtest;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(Resource, Default)]
struct ShouldRun(bool);

fn should_run(should_run: Res<ShouldRun>) -> bool {
    should_run.0
}

// generates: app.configure_sets(Update, (TestSet::A, TestSet::B, TestSet::C).chain());
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
#[auto_configure_system_set(plugin = TestPlugin, schedule = Update, chain, config(run_if = should_run))]
enum TestSet {
    A,
    B,
    #[auto_configure_system_set_config(exclude)]
    C,
}

#[derive(Resource, Default)]
struct RunLog(Vec<&'static str>);

impl RunLog {
    fn push(&mut self, s: &'static str) {
        self.0.push(s);
    }
}

macro_rules! run_log_system {
    ($ident:ident) => {
        fn $ident(mut run_log: ResMut<RunLog>) {
            run_log.push(stringify!($ident));
        }
    };
}

run_log_system!(a);
run_log_system!(b);
run_log_system!(c);

fn app() -> App {
    let mut app = internal_test_util::create_minimal_app();
    app.add_plugins(TestPlugin);
    app.init_resource::<RunLog>();
    app.init_resource::<ShouldRun>();
    app.add_systems(Update, a.in_set(TestSet::A));
    app.add_systems(Update, b.in_set(TestSet::B));
    app.add_systems(Update, c.in_set(TestSet::C));
    app
}

fn get_run_log(app: &App) -> &Vec<&'static str> {
    &app.world().resource::<RunLog>().0
}
fn enable_run(app: &mut App) {
    app.world_mut().resource_mut::<ShouldRun>().0 = true;
}

macro_rules! expect_vec_ref {
    ($($val:tt),* $(,)?) => {
        &Vec::<&'static str>::from([$(stringify!($val)),*])
    };
}

#[xtest]
fn test_auto_configure_system_set() {
    let mut app = app();
    assert!(get_run_log(&app).is_empty());
    app.update();
    assert_eq!(get_run_log(&app), expect_vec_ref!());
    enable_run(&mut app);
    app.update();
    assert_eq!(get_run_log(&app), expect_vec_ref!(a, b, c));
}
