# Bevy Auto Plugin

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/StrikeForceZero/bevy_auto_plugin#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Downloads](https://img.shields.io/crates/d/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Docs](https://docs.rs/bevy_auto_plugin/badge.svg)](https://docs.rs/bevy_auto_plugin/latest/bevy_auto_plugin/)
[![CI](https://github.com/StrikeForceZero/bevy_auto_plugin/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/StrikeForceZero/bevy_auto_plugin/actions?query=branch%3Amain)

Bevy Auto Plugin provides attribute macros that automatically handle the repetitive setup usually required in Bevy plugins. 
Instead of manually wiring up components, resources, events, states, and systems - and remembering all their respective derives - you can declare them with concise annotations tied to a plugin. 

If you’ve ever added several components only to hit runtime errors or discover a missing `TypeRegistry` entry when using tools like `bevy-inspector-egui`, this plugin is for you.
It helps keep your code ***focused on game logic rather than framework plumbing.***

The following examples demonstrate how common Bevy patterns can be expressed more ergonomically with `#[auto_*]` macros, while still generating the underlying bevy-specific code you would normally write by hand.

## Examples

### Basic

#### Component
instead of having to specify all these derives and remember to reflect:
```rust
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Debug, Default)]
#[require(Name::new("FooComponent"))]
struct FooComponent;
```
and then later having to remember to register your component in the type registry:
```rust
struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FooComponent>();
    }
}
```
you can do:
```rust
#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_component(
    plugin = MyPlugin,
    derive(Debug, Default),
    reflect(Debug, Default),
    register,
    auto_name,
)]
struct FooComponent;
```
#### System
instead of writing a function then scheduling in your plugin's build function:
```rust
fn my_system() {}

fn plugin(app: &mut App) {
    app.add_systems(Update, my_system.run_if(some_condition).after(some_other_system));
}
```
you can do it via the `auto_system(..)` attribute macro:
```rust
#[auto_system(
    plugin = MyPlugin,
    schedule = Update,
    config( 
        run_if = some_condition,
        after = some_other_system,
    ),
)]
fn my_system() {}
```

### Generics
#### Component
if your items have generics, you can specify the types using the `generics(...)` meta argument for each concrete type. 
```rust
#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[auto_component(
    plugin = MyPlugin,
    generics(usize, bool),
    generics(bool, bool),
    derive(Debug, Default),
    reflect(Debug, Default),
    register,
    auto_name,
)]
struct FooComponent<A, B>(A, B);
```
this will generate something equivalent to:
```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FooComponent<usize, bool>>();
        app.register_type::<FooComponent<bool, bool>>();
    }
}
```
#### System
if your systems have generics, you can specify the types using the `generics(...)` meta argument for each concrete type.
```rust
#[auto_system(
    plugin = MyPlugin,
    schedule = Update, 
    generics(Name), 
    generics(Transform),
    config( 
        run_if = some_condition,
        after = some_other_system,
    ),
)]
fn my_system<A: Component>(q: Query<&A>) {
    for item in q.iter() {
        //
    }
}
```
this will generate something equivalent to:
```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_system::<Name>.run_if(some_condition).after(some_other_system));
        app.add_systems(Update, my_system::<Transform>.run_if(some_condition).after(some_other_system));
    }
}
```

### Plugin

There are three distinct ways to make a bindable plugin:

```rust
#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;
```

```rust
#[derive(AutoPlugin)]
struct MyPlugin;

impl Plugin for MyPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        //
    }
}
```

```rust
#[derive(AutoPlugin)]
struct MyPlugin;

#[auto_plugin(plugin = MyPlugin)]
fn plugin(app: &mut App) {
    //
}
```

There is `auto_plugin` arguments if your plugin has generics.

See [tests](tests/global) for other examples

### Expanded

If you were looking to cherry-pick certain functionality like `auto_name` or `auto_register_type` for example you could use them individually:
Only requirement when using global mode is you need tp make sure you are binding to a plugin that derives `AutoPlugin`

#### Global Mode

Features required:
- `default` or `mode_global` or `all_modes`

```rust
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_name(plugin = MyPlugin)]
struct FooComponent;

// or if you want to omit plugin for each auto_* item:
#[auto_bind_plugin(plugin = MyPlugin)]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_register_type]
#[auto_name]
struct FooComponent2;

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_init_resource(plugin = MyPlugin)]
struct FooDefaultResource(usize);

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_init_resource(plugin = MyPlugin)]
#[auto_insert_resource(plugin = MyPlugin, resource(FooResource(1)))]
struct FooResource(usize);

#[derive(Message, Debug, Default, Reflect)]
#[auto_register_type(plugin = MyPlugin)]
#[auto_add_message(plugin = MyPlugin)]
struct FooEvent(usize);

#[derive(States, Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
#[auto_init_state(plugin = MyPlugin)]
#[auto_register_state_type(plugin = MyPlugin)]
enum FooState {
    #[default]
    Start,
    End,
}

#[auto_add_system(plugin = MyPlugin, schedule = Update)]
fn foo_system(mut foo_resource: ResMut<FooResource>) {
    foo_resource.0 += 1;
}

fn main() {
    App::new()
        .add_plugins(MyPlugin)
        // ... other plugins and setup
        .run();
}
```

Which automatically implements the Plugin trait for `MyPlugin` and registers all the types, resources, events, and systems when the plugin is added to the app.

#### Known Limitations
- WASM should work, CI uses the `wasm-bindgen-test-runner` but maybe there's a specific wasm target/environment where it fails?

---

## [Changelog](CHANGELOG.md)
## [Migrations](MIGRATIONS.md)

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

### Your Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.