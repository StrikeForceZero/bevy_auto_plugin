pub mod module {
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_plugin;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_add_event;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_init_resource;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_init_state;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_name;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_register_type;
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_register_state_type;
}

pub mod inline {
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_plugin;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_add_event;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_init_resource;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_init_state;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_name;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_register_type;
    #[doc(inline)]
    pub use bevy_auto_plugin_nightly_proc_macros::auto_register_state_type;
}