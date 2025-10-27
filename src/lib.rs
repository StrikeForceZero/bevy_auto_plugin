#[doc(hidden)]
pub mod __private {
    pub use bevy_auto_plugin_shared as shared;
}

pub mod prelude {
    #[doc = include_str!("../docs/proc_attributes/derive_auto_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::AutoPlugin;

    #[doc = include_str!("../docs/proc_attributes/auto_add_message.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_message;

    #[doc = include_str!("../docs/proc_attributes/auto_add_plugin.md")]
    #[deprecated(since = "0.6.0", note = "Use `auto_add_message` instead.")]
    pub use bevy_auto_plugin_proc_macros::auto_add_message as auto_add_event;

    #[doc = include_str!("../docs/proc_attributes/auto_add_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_plugin;

    #[doc = include_str!("../docs/proc_attributes/auto_add_system.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_system;

    #[doc = include_str!("../docs/proc_attributes/auto_init_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_resource;

    #[doc = include_str!("../docs/proc_attributes/auto_init_state.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_state;

    #[doc = include_str!("../docs/proc_attributes/auto_init_sub_state.md")]
    pub use bevy_auto_plugin_proc_macros::auto_init_sub_state;

    #[doc = include_str!("../docs/proc_attributes/auto_insert_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_insert_resource;

    #[doc = include_str!("../docs/proc_attributes/auto_name.md")]
    pub use bevy_auto_plugin_proc_macros::auto_name;

    #[doc = include_str!("../docs/proc_attributes/auto_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_plugin;

    #[doc = include_str!("../docs/proc_attributes/auto_register_state_type.md")]
    pub use bevy_auto_plugin_proc_macros::auto_register_state_type;

    #[doc = include_str!("../docs/proc_attributes/auto_register_type.md")]
    pub use bevy_auto_plugin_proc_macros::auto_register_type;

    #[doc = include_str!("../docs/proc_attributes/auto_add_observer.md")]
    pub use bevy_auto_plugin_proc_macros::auto_add_observer;

    #[doc = include_str!("../docs/proc_attributes/auto_component.md")]
    pub use bevy_auto_plugin_proc_macros::auto_component;

    #[doc = include_str!("../docs/proc_attributes/auto_resource.md")]
    pub use bevy_auto_plugin_proc_macros::auto_resource;

    #[doc = include_str!("../docs/proc_attributes/auto_event.md")]
    pub use bevy_auto_plugin_proc_macros::auto_event;

    #[doc = include_str!("../docs/proc_attributes/auto_message.md")]
    pub use bevy_auto_plugin_proc_macros::auto_message;

    #[doc = include_str!("../docs/proc_attributes/auto_states.md")]
    pub use bevy_auto_plugin_proc_macros::auto_states;

    #[doc = include_str!("../docs/proc_attributes/auto_system.md")]
    pub use bevy_auto_plugin_proc_macros::auto_system;

    #[doc = include_str!("../docs/proc_attributes/auto_observer.md")]
    pub use bevy_auto_plugin_proc_macros::auto_observer;

    #[doc = include_str!("../docs/proc_attributes/auto_run_on_build.md")]
    pub use bevy_auto_plugin_proc_macros::auto_run_on_build;

    #[doc = include_str!("../docs/proc_attributes/auto_bind_plugin.md")]
    pub use bevy_auto_plugin_proc_macros::auto_bind_plugin;

    #[doc = include_str!("../docs/proc_attributes/auto_configure_system_set.md")]
    pub use bevy_auto_plugin_proc_macros::auto_configure_system_set;
}
