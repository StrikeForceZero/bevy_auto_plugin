use bevy_app::prelude::*;
use bevy_auto_plugin_proc_macros::AutoPlugin;
use bevy_auto_plugin_proc_macros::global_auto_plugin;
use bevy_ecs::prelude::*;

#[derive(AutoPlugin)]
#[global_auto_plugin(generics(u32, i32), impl_plugin_trait)]
struct Test<T1, T2>(T1, T2)
where
    T1: Send + Sync + 'static,
    T2: Send + Sync + 'static;
