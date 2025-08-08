use bevy_auto_plugin::modes::flat_file::prelude::*;

struct Test;

impl Test {
    #[auto_plugin]
    fn plugin(&self) {}
}

// dummy main
fn main() {}
