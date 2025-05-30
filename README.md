# Bevy Auto Plugin

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/StrikeForceZero/bevy_auto_plugin#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Downloads](https://img.shields.io/crates/d/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Docs](https://docs.rs/bevy_auto_plugin/badge.svg)](https://docs.rs/bevy_auto_plugin/latest/bevy_auto_plugin/)
[![CI](https://github.com/StrikeForceZero/bevy_auto_plugin/workflows/CI/badge.svg)](https://github.com/StrikeForceZero/bevy_auto_plugin/actions)

This crate is designed to reduce the boilerplate required when creating Bevy plugins. Typically, you need to manually register types, initialize resources, and add events. With these macros, you can streamline the process by simply adding the usual derives and attribute macros to your items. As long as you invoke your plugin’s build function, the manual steps are handled automatically.

While there are ongoing discussions about auto-registering types by default in Bevy—potentially making part of this crate redundant—the remaining functionality should continue to provide quality-of-life improvements for bevy related development.

## Usage - Stable
```rust
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin_module::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;
    
    #[auto_register_type]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    #[auto_name]
    pub struct FooComponent;

    #[auto_register_type(FooComponentWithGeneric<bool>)]
    #[auto_register_type(FooComponentWithGeneric<u32>)]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct FooComponentWithGeneric<T>(T);

    #[auto_register_type]
    #[auto_add_event]
    #[derive(Event, Reflect)]
    pub struct FooEvent;

    #[auto_register_type(FooEvent<bool>)]
    #[auto_add_event]
    #[derive(Event, Reflect)]
    pub struct FooEventWithGeneric<T>(T);

    #[auto_register_type]
    #[auto_init_resource]
    #[derive(Resource, Default, Reflect)]
    #[reflect(Resource)]
    pub struct FooResource;

    #[auto_register_type(FooResourceWithGeneric<bool>)]
    #[auto_init_resource]
    #[derive(Resource, Default, Reflect)]
    #[reflect(Resource)]
    pub struct FooResourceWithGeneric<T>(T);
}

fn plugin(app: &mut App) {
    plugin_module::init(app);
}
```

Which generates this code
```rust
mod plugin_module {
    // ...
    fn init(app: &mut App) {
        app.register_type::<FooComponent>();
        app.register_type::<FooComponentWithGeneric<bool>>();
        app.register_type::<FooComponentWithGeneric<u32>>();
        app.register_type::<FooEvent>();
        app.register_type::<FooEventWithGeneric<bool>>();
        app.register_type::<FooResource>();
        app.register_type::<FooResourceWithGeneric<bool>>();

        app.add_event::<FooEvent>();
        app.add_event::<FooEventWithGeneric<bool>>();

        app.init_resource::<FooResource>();
        app.init_resource::<FooResourceWithGeneric<bool>>();

        app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
    }
}
```

### Known Limitations
- Causes issues for ide's like RustRover

## Usage - Nightly

NOTE:
`nightly-2025-04-16` [changed the api](https://github.com/rust-lang/rust/issues/54725#event-2307701034) being used to track files. As of writing the [language server is stubbed to return none](https://github.com/rust-lang/rust/pull/139671#issuecomment-2796920999). 

Use `nightly-2025-04-15` or earlier along with `--feature=bevy_auto_plugin/nightly_pre_2025_04_16`  or `bevy_auto_plugin = { features=["nightly_pre_2025_04_16"] }` to use previous API.

Otherwise:

`--features=bevy_auto_plugin/nightly` or `bevy_auto_plugin = { features=["nightly"] }`



```rust
use bevy::prelude::*;
use bevy_auto_plugin::auto_plugin::*;

#[auto_register_type]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_name]
struct FooComponent;

#[auto_register_type(FooComponentWithGeneric<bool>)]
#[auto_register_type(FooComponentWithGeneric<u32>)]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct FooComponentWithGeneric<T>(T);

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Reflect)]
struct FooEvent;

#[auto_register_type(FooEvent<bool>)]
#[auto_add_event]
#[derive(Event, Reflect)]
struct FooEventWithGeneric<T>(T);

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct FooResource;

#[auto_register_type(FooResourceWithGeneric<bool>)]
#[auto_init_resource]
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct FooResourceWithGeneric<T>(T);

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {}
```

Which generates this code in your fn accepting `&mut App`
```rust
#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    app.register_type::<FooComponent>();
    app.register_type::<FooComponentWithGeneric<bool>>();
    app.register_type::<FooComponentWithGeneric<u32>>();
    app.register_type::<FooEvent>();
    app.register_type::<FooEventWithGeneric<bool>>();
    app.register_type::<FooResource>();
    app.register_type::<FooResourceWithGeneric<bool>>();
    
    app.add_event::<FooEvent>();
    app.add_event::<FooEventWithGeneric<bool>>();
    
    app.init_resource::<FooResource>();
    app.init_resource::<FooResourceWithGeneric<bool>>();

    app.register_required_components_with::<FooComponent, Name>(|| Name::new("FooComponent"));
    // ...
}
```

### Known Limitations
- The internal state relies on call site file paths which currently requires `Nightly` rust.

- All items need to be in the same module. This won't work:
```rust
use bevy::prelude::*;
use bevy_auto_plugin::*;
mod foo {
    use super::*;
    #[auto_register_type]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    struct FooComponent;
}

#[auto_plugin(app=app)]
fn plugin(app: &mut App) {
    // ...
}
```

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

### Your Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.