mod actions;
mod auto_plugin;
mod rewrites;

pub mod prelude {
    pub use super::auto_plugin::{
        AutoPluginFnArgs, AutoPluginStructOrEnumArgs, resolve_app_param_name,
    };
    pub use crate::macro_api::attributes::actions::auto_add_message::AddMessageArgs;
    pub use crate::macro_api::attributes::actions::auto_add_observer::AddObserverArgs;
    pub use crate::macro_api::attributes::actions::auto_add_system::AddSystemArgs;
    pub use crate::macro_api::attributes::actions::auto_init_resource::InitResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_init_state::InitStateArgs;
    pub use crate::macro_api::attributes::actions::auto_insert_resource::InsertResourceArgs;
    pub use crate::macro_api::attributes::actions::auto_name::NameArgs;
    pub use crate::macro_api::attributes::actions::auto_register_state_type::RegisterStateTypeArgs;
    pub use crate::macro_api::attributes::actions::auto_register_type::RegisterTypeArgs;
    pub use crate::macro_api::attributes::rewrites::auto_component::ComponentArgs;
    pub use crate::macro_api::attributes::rewrites::auto_event::EventArgs;
    pub use crate::macro_api::attributes::rewrites::auto_message::MessageArgs;
    pub use crate::macro_api::attributes::rewrites::auto_observer::ObserverArgs;
    pub use crate::macro_api::attributes::rewrites::auto_resource::ResourceArgs;
    pub use crate::macro_api::attributes::rewrites::auto_states::StatesArgs;
    pub use crate::macro_api::attributes::rewrites::auto_system::SystemArgs;
}
