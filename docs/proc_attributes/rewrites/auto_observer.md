Automatically adds a global observer

# Parameters

Parameters marked **repeatable** may appear more than once within the same attribute.
All other parameters may appear at most once.

- `plugin = PluginType` - Required unless the `default_plugin` feature is enabled and `#[auto_plugin(default_plugin)]` is in scope. Specifies which plugin should register this observer.
- `after_build` - Optional. Injects this macro's tokens at the end of the plugin build instead of the start.
- `generics(T1, T2, ...)` - Optional, repeatable. Specifies one combination of concrete types for generic parameters. Repeat for additional combinations.
  Note: Clippy will complain if you have duplicate generic type names. For those you can use named generics: `generics(A = ..., B = ...)`.
- `config(..)` - Optional, non-repeatable. Contains the configuration parameters below.
  - `run_if = Condition` - Optional, repeatable. Adds a Bevy observer run condition.

# Example
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component)]
struct Foo;

#[auto_observer(plugin = MyPlugin)]
fn foo_observer(add: On<Add<Foo>>, mut commands: Commands) {
    // ...
}
```

# Example (with run condition)
```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component)]
struct Foo;

fn should_observe() -> bool {
    true
}

#[auto_observer(plugin = MyPlugin, config(run_if = should_observe))]
fn foo_observer(add: On<Add<Foo>>, mut commands: Commands) {
    // ...
}
```
