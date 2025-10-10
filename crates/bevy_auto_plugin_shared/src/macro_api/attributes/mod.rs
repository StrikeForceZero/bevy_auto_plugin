mod auto_add_message;
mod auto_add_observer;
mod auto_add_system;
mod auto_component;
mod auto_event;
mod auto_init_resource;
mod auto_init_state;
mod auto_insert_resource;
mod auto_message;
mod auto_name;
mod auto_observer;
mod auto_plugin;
mod auto_register_state_type;
mod auto_register_type;
mod auto_resource;
mod auto_states;
mod auto_system;

pub mod prelude {
    pub use {
        super::auto_add_message::AddMessageArgs,
        super::auto_add_observer::AddObserverArgs,
        super::auto_add_system::AddSystemArgs,
        super::auto_component::ComponentArgs,
        super::auto_event::EventArgs,
        super::auto_init_resource::InitResourceArgs,
        super::auto_init_state::InitStateArgs,
        super::auto_insert_resource::InsertResourceArgs,
        super::auto_message::MessageArgs,
        super::auto_name::NameArgs,
        super::auto_observer::ObserverArgs,
        super::auto_plugin::{
            AutoPluginFnArgs, AutoPluginStructOrEnumArgs, resolve_app_param_name,
        },
        super::auto_register_state_type::RegisterStateTypeArgs,
        super::auto_register_type::RegisterTypeArgs,
        super::auto_resource::ResourceArgs,
        super::auto_states::StatesArgs,
        super::auto_system::SystemArgs,
    };
}
