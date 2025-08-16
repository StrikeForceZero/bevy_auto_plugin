use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_register_type(generics(u32))]
struct Test<T1, T2>(T1, T2);

#[auto_plugin(app_param=_app)]
fn plugin(_app: &mut bevy_app::App) {}

// dummy main
fn main() {}
