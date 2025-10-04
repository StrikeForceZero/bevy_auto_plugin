pub mod add_message;
pub mod add_observer;
pub mod add_system;
pub mod auto_name;
pub mod auto_plugin;
pub mod init_resource;
pub mod init_state;
pub mod insert_resource;
pub mod register_state_type;
pub mod register_type;
pub mod shorthand;

pub mod prelude {
    pub use super::*;
    pub use add_message::AddMessageAttributeArgs;
    pub use add_observer::AddObserverAttributeArgs;
    pub use add_system::AddSystemAttributeArgs;
    pub use auto_name::AutoNameAttributeArgs;
    pub use init_resource::InitResourceAttributeArgs;
    pub use init_state::InitStateAttributeArgs;
    pub use insert_resource::InsertResourceAttributeArgs;
    pub use register_state_type::RegisterStateTypeAttributeArgs;
    pub use register_type::RegisterTypeAttributeArgs;
}
