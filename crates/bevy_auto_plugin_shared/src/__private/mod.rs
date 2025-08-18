pub mod attribute;
pub mod attribute_args;
pub mod context;
mod expr_value;
mod generics;
pub mod item_with_attr_match;
mod macros;
pub mod modes;
mod type_list;
pub mod util;

pub use bevy_app;
pub use bevy_ecs;
pub use bevy_ecs_macros;
pub use bevy_log;
pub use bevy_reflect;
pub use bevy_reflect_derive;
pub use bevy_state;

// module to allow single item globs
pub mod reflect {
    pub mod std_traits {
        pub use bevy_reflect::std_traits::ReflectDefault;
    }
    pub mod component {
        pub use bevy_ecs::reflect::ReflectComponent;
    }
    pub mod resource {
        pub use bevy_ecs::reflect::ReflectResource;
    }
}
