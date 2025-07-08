use bevy_auto_plugin::flat_file::prelude::*;

struct Test;

impl Test {
    #[auto_plugin]
    fn plugin(&self) {}
}

// dummy main
fn main() {}
