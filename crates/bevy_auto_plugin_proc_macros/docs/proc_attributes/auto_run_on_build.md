Automatically runs the `fn(&mut App) -> ()` when the `Plugin::build` is called.

# Parameters
- `plugin = PluginType` - Required. Specifies which plugin should run this fn on build.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the fn will be run for each of these specific generic parameters.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_run_on_build(plugin = MyPlugin)]
fn run_this(_app: &mut App) {
    
}

// This will run run_this when MyPlugin::build is called
```

# Example (with generics)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

struct Foo;
struct Bar;

pub trait Thing {
    const STUFF: &str;
}

impl Thing for Foo {
    const STUFF: &str = "Foo stuff";
}

impl Thing for Bar {
    const STUFF: &str = "Bar stuff";
}

#[auto_run_on_build(plugin = MyPlugin, generics(Foo), generics(Bar))]
fn run_this<T: Thing>(_app: &mut App) {
    let stuff = T::STUFF;
    println!("{stuff}");
}

// This will run_this::<Foo>() and run_this::<Bar>()
// when MyPlugin::build is called
```