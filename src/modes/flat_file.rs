#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub mod prelude {
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_message as auto_add_message;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_system as auto_add_system;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_init_resource as auto_init_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_init_state as auto_init_state;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_insert_resource as auto_insert_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_name as auto_name;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_plugin as auto_plugin;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_register_state_type as auto_register_state_type;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_register_type as auto_register_type;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_add_observer as auto_add_observer;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_component as auto_component;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_resource as auto_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_event as auto_event;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_states as auto_states;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_system as auto_system;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::flat_file_auto_observer as auto_observer;
}
