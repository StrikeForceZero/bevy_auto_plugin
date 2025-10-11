#[doc(hidden)]
pub mod __private {
    pub use bevy_auto_plugin_shared as shared;
}

pub mod prelude {
    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::AutoPlugin;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_add_message;

    #[doc(inline)]
    #[deprecated(since = "0.6.0", note = "Use `auto_add_message` instead.")]
    pub use bevy_auto_plugin_proc_macros::auto_add_message as auto_add_event;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_add_system;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_init_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_init_state;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_insert_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_name;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_plugin;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_register_state_type;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_register_type;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_add_observer;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_component;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_resource;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_event;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_message;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_states;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_system;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_observer;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_run_on_build;

    #[doc(inline)]
    pub use bevy_auto_plugin_proc_macros::auto_bind_plugin;
}

#[deprecated(since = "0.7.0", note = "bevy_auto_plugin::prelude::* instead")]
pub mod modes {
    #[deprecated(since = "0.7.0", note = "bevy_auto_plugin::prelude::* instead")]
    pub mod global {
        #[deprecated(since = "0.7.0", note = "bevy_auto_plugin::prelude::* instead")]
        pub mod prelude {
            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::AutoPlugin;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_add_message;

            #[doc(inline)]
            #[deprecated(since = "0.6.0", note = "Use `auto_add_message` instead.")]
            pub use bevy_auto_plugin_proc_macros::auto_add_message as auto_add_event;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_add_system;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_init_resource;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_init_state;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_insert_resource;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_name;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_plugin;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_register_state_type;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_register_type;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_add_observer;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_component;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_resource;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_event;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_message;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_states;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_system;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_observer;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_run_on_build;

            #[doc(inline)]
            pub use bevy_auto_plugin_proc_macros::auto_bind_plugin;
        }
    }
}
