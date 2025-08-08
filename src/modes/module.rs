pub mod prelude {
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_add_event as auto_add_event;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_add_system as auto_add_system;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_init_resource as auto_init_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_init_state as auto_init_state;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_insert_resource as auto_insert_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_name as auto_name;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_plugin as auto_plugin;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_register_state_type as auto_register_state_type;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::module_auto_register_type as auto_register_type;
}
