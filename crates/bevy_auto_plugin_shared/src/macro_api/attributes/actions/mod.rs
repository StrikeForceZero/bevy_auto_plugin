mod auto_add_message;
mod auto_add_observer;
mod auto_add_plugin;
mod auto_add_system;
mod auto_configure_system_set;
mod auto_init_resource;
mod auto_init_state;
mod auto_init_sub_state;
mod auto_insert_resource;
mod auto_name;
mod auto_plugin_custom;
mod auto_register_state_type;
mod auto_register_type;
mod auto_run_on_build;

pub mod prelude {
    use super::*;
    pub use auto_add_message::*;
    pub use auto_add_observer::*;
    pub use auto_add_plugin::*;
    pub use auto_add_system::*;
    pub use auto_configure_system_set::*;
    pub use auto_init_resource::*;
    pub use auto_init_state::*;
    pub use auto_init_sub_state::*;
    pub use auto_insert_resource::*;
    pub use auto_name::*;
    pub use auto_plugin_custom::*;
    pub use auto_register_state_type::*;
    pub use auto_register_type::*;
    pub use auto_run_on_build::*;
}
