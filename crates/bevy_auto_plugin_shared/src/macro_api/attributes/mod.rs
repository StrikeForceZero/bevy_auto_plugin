mod auto_add_message;
mod auto_add_observer;
mod auto_add_system;
mod auto_auto_name;
mod auto_component;
mod auto_event;
mod auto_init_resource;
mod auto_init_state;
mod auto_insert_resource;
mod auto_message;
mod auto_observer;
mod auto_plugin;
mod auto_register_state_type;
mod auto_register_type;
mod auto_resource;
mod auto_states;
mod auto_system;

pub mod prelude {
    pub use {
        super::auto_add_message::AddMessageAttributeArgs,
        super::auto_add_observer::AddObserverAttributeArgs,
        super::auto_add_system::AddSystemAttributeArgs,
        super::auto_auto_name::AutoNameAttributeArgs,
        super::auto_component::ComponentAttributeArgs,
        super::auto_event::EventAttributeArgs,
        super::auto_init_resource::InitResourceAttributeArgs,
        super::auto_init_state::InitStateAttributeArgs,
        super::auto_insert_resource::InsertResourceAttributeArgs,
        super::auto_message::MessageAttributeArgs,
        super::auto_observer::ObserverAttributeArgs,
        super::auto_plugin::{
            AutoPluginFnAttributeArgs, AutoPluginStructOrEnumAttributeArgs, resolve_app_param_name,
        },
        super::auto_register_state_type::RegisterStateTypeAttributeArgs,
        super::auto_register_type::RegisterTypeAttributeArgs,
        super::auto_resource::ResourceAttributeArgs,
        super::auto_states::StatesAttributeArgs,
        super::auto_system::SystemAttributeArgs,
    };
}
