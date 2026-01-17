//! # Bevy Auto Plugin
//! [GitHub repository](https://github.com/strikeforcezero/bevy_auto_plugin)
//!
//! ## Getting Started:
//!
//! ### Plugin
//!
//! There are three distinct ways to make a bindable plugin:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//!
//! #[derive(AutoPlugin)]
//! #[auto_plugin(impl_plugin_trait)]
//! struct MyPlugin;
//! ```
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//!
//! #[derive(AutoPlugin)]
//! struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     #[auto_plugin]
//!     fn build(&self, app: &mut App) {
//!         //
//!     }
//! }
//! ```
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//!
//! #[derive(AutoPlugin)]
//! struct MyPlugin;
//!
//! #[auto_plugin(plugin = MyPlugin)]
//! fn plugin(app: &mut App) {
//!     //
//! }
//! ```
//!
//! ### Using Attributes
//! When `Plugin::build` is called on `MyPlugin` (i.e., `app.add_plugins(MyPlugin)`), the code for each attribute will be executed.
//!
//! You can use the `auto_*` attributes in several different ways:
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//!
//! #[derive(AutoPlugin)]
//! #[auto_plugin(impl_plugin_trait)]
//! struct MyPlugin;
//!
//! #[auto_component(
//!     plugin = MyPlugin,
//!     derive(Debug, Default),
//!     reflect(Debug, Default),
//!     register,
//!     auto_name,
//! )]
//! struct FooComponent;
//! ```
//!
//! which gets rewritten into:
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//!
//! #[derive(AutoPlugin)]
//! #[auto_plugin(impl_plugin_trait)]
//! struct MyPlugin;
//!
//! #[derive(Component, Reflect, Debug, Default)]
//! #[reflect(Component, Debug, Default)]
//! #[auto_name(plugin = MyPlugin)]
//! #[auto_register_type(plugin = MyPlugin)]
//! struct FooComponent;
//! ```
//!
//! or maybe you want a template:
//! ```rust
//! use bevy::prelude::*;
//! use bevy_auto_plugin::prelude::*;
//! use meta_merge::*;
//!
//! #[derive(AutoPlugin)]
//! #[auto_plugin(impl_plugin_trait)]
//! struct MyPlugin;
//!
//! #[export(copy(prepend))]
//! #[derive(Component, Reflect, Debug, Default)]
//! #[reflect(Component, Debug, Default)]
//! #[auto_name(plugin = MyPlugin)]
//! #[auto_register_type(plugin = MyPlugin)]
//! struct DefaultComponentTemplate;
//!
//! #[apply(CopyDefaultComponentTemplate!)]
//! struct FooComponent;
//!
//! #[apply(CopyDefaultComponentTemplate!)]
//! struct BarComponent;
//! ```

/// Private Re-exports
#[doc(hidden)]
pub mod __private {
    pub use bevy_auto_plugin_shared as shared;
}

pub mod prelude {
    #[doc = include_str!("../docs/derives/AutoPlugin.md")]
    pub use bevy_auto_plugin_proc_macros::AutoPlugin;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_add_message.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_message;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_add_plugin.md")]
    #[deprecated(since = "0.6.0", note = "Use `auto_add_message` instead.")]
    pub use bevy_auto_plugin_proc_macros::auto_add_message as auto_add_event;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_add_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_plugin;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_add_system.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_system;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_init_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_resource;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_init_state.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_state;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_init_sub_state.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_sub_state;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_insert_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_insert_resource;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_name.md")]
    pub use bevy_auto_plugin_proc_macros::auto_name;

    #[doc = include_str!("../docs/proc_attributes/auto_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_plugin;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_register_state_type.md")]
    pub use bevy_auto_plugin_proc_macros::auto_register_state_type;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_register_type.md")]
    pub use bevy_auto_plugin_proc_macros::auto_register_type;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_add_observer.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_observer;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_component.md")]
    pub use bevy_auto_plugin_proc_macros::auto_component;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_resource;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_event.md")]
    pub use bevy_auto_plugin_proc_macros::auto_event;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_message.md")]
    pub use bevy_auto_plugin_proc_macros::auto_message;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_states.md")]
    pub use bevy_auto_plugin_proc_macros::auto_states;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_sub_states.md")]
    pub use bevy_auto_plugin_proc_macros::auto_sub_states;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_system.md")]
    pub use bevy_auto_plugin_proc_macros::auto_system;

    #[doc = include_str!("../docs/proc_attributes/rewrites/auto_observer.md")]
    pub use bevy_auto_plugin_proc_macros::auto_observer;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_run_on_build.md")]
    pub use bevy_auto_plugin_proc_macros::auto_run_on_build;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_bind_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_bind_plugin;

    #[doc = include_str!("../docs/proc_attributes/actions/auto_configure_system_set.md")]
    pub use bevy_auto_plugin_proc_macros::auto_configure_system_set;

    pub use bevy_auto_plugin_proc_macros::auto_plugin_build_hook;
}
