Automatically runs the `fn(&mut App) -> ()` when the `Plugin::build` is called.

# Parameters
- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should run this fn on build.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional. Specifies concrete types for generic parameters.
  When provided, the fn will be run for each of these specific generic parameters.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(T1 = ..., T2 = ...)`.

# Notes
- This attribute can be applied to a `use` item; each imported name becomes its own target.
- `use ...::*`, `use ...::self`, and `_` imports are not supported.
- Renames (`as`) are supported and use the local name.
- Registry entries are sorted by file/line/column; within a file, definition order is preserved. Across files, order follows file path, so use `after_build` or explicit plugin ordering when order matters.

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
