mod auto_component;
mod auto_event;
mod auto_message;
mod auto_observer;
mod auto_resource;
mod auto_states;
mod auto_sub_states;
mod auto_system;

pub mod prelude {
    pub use super::*;
    pub use auto_component::*;
    pub use auto_event::*;
    pub use auto_message::*;
    pub use auto_observer::*;
    pub use auto_resource::*;
    pub use auto_states::*;
    pub use auto_sub_states::*;
    pub use auto_system::*;
}
