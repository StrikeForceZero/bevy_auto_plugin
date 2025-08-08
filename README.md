# Bevy Auto Plugin

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/StrikeForceZero/bevy_auto_plugin#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Downloads](https://img.shields.io/crates/d/bevy_auto_plugin.svg)](https://crates.io/crates/bevy_auto_plugin)
[![Docs](https://docs.rs/bevy_auto_plugin/badge.svg)](https://docs.rs/bevy_auto_plugin/latest/bevy_auto_plugin/)
[![CI](https://github.com/StrikeForceZero/bevy_auto_plugin/workflows/CI/badge.svg)](https://github.com/StrikeForceZero/bevy_auto_plugin/actions)

This crate is designed to reduce the boilerplate required when creating Bevy plugins. Typically, you need to manually register types, initialize resources, and add events. With these macros, you can streamline the process by simply adding the usual derives and attribute macros to your items. As long as you invoke your plugin’s build function, the manual steps are handled automatically.

While there are ongoing discussions about auto-registering types by default in Bevy—potentially making part of this crate redundant—the remaining functionality should continue to provide quality-of-life improvements for bevy related development.

## Usage - Module
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::module::prelude::*;

#[auto_plugin(init_name=init)]
mod plugin_module {
    use super::*;
    
    #[auto_register_type]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    #[auto_name]
    pub struct FooComponent;

    #[auto_register_type(generics(bool))]
    #[auto_register_type(generics(u32))]
    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct FooComponentWithGeneric<T>(T);

    #[auto_register_type]
    #[auto_add_event]
    #[derive(Event, Reflect)]
    pub struct FooEvent;

    #[auto_register_type(generics(bool))]
    #[auto_add_event]
    #[derive(Event, Reflect)]
    pub struct FooEventWithGeneric<T>(T);

    #[auto_register_type]
    #[auto_init_resource]
    #[derive(Resource, Default, Reflect)]
    #[reflect(Resource)]
    pub struct FooResource;

    #[auto_register_type(generics(bool))]
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

## Usage - Flat File

```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

#[auto_register_type]
#[derive(Component, Reflect)]
#[reflect(Component)]
#[auto_name]
struct FooComponent;

#[auto_register_type(generics(bool))]
#[auto_register_type(generics(u32))]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct FooComponentWithGeneric<T>(T);

#[auto_register_type]
#[auto_add_event]
#[derive(Event, Reflect)]
struct FooEvent;

#[auto_register_type(generics(bool))]
#[auto_add_event]
#[derive(Event, Reflect)]
struct FooEventWithGeneric<T>(T);

#[auto_register_type]
#[auto_init_resource]
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct FooResource;

#[auto_register_type(generics(bool))]
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
- Won't provide outputs in IDE's due to [Language Server Stubbed](https://github.com/rust-lang/rust/blob/4e973370053a5fe87ee96d43c506623e9bd1eb9d/src/tools/rust-analyzer/crates/proc-macro-srv/src/server_impl/rust_analyzer_span.rs#L144-L147)
  - use `lang_server_noop` feature (enabled by default) to allow `flat_file` macros to no-ops when they fail to resolve `Span::local_file`
  - attempts to naively detect when running under `rustc` context to otherwise bubble up the errors to the compiler
- All items need to be in the same module. This won't work:
```rust
use bevy::prelude::*;
use bevy_auto_plugin::modes::flat_file::prelude::*;

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