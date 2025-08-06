use crate::attribute_args::{AddSystemSerializedArgs, InsertResourceSerializedArgsWithPath};
use proc_macro2::{Ident, Span};
use std::collections::HashSet;
use std::hash::Hash;

pub mod attribute_args;
pub mod bevy_app_code_gen;
mod expr_value;
pub mod modes;
mod type_list;
pub mod util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginAttribute {
    RegisterType,
    AddEvent,
    InitResource,
    InsertResource,
    InitState,
    Name,
    RegisterStateType,
    AddSystem,
}

impl AutoPluginAttribute {
    pub const fn ident_str(self) -> &'static str {
        match self {
            Self::RegisterType => "auto_register_type",
            Self::AddEvent => "auto_add_event",
            Self::InitResource => "auto_init_resource",
            Self::InsertResource => "auto_insert_resource",
            Self::InitState => "auto_init_state",
            Self::Name => "auto_name",
            Self::RegisterStateType => "auto_register_state_type",
            Self::AddSystem => "auto_add_system",
        }
    }
}

#[derive(Default)]
pub struct AutoPluginContext {
    pub register_types: HashSet<String>,
    pub register_state_types: HashSet<String>,
    pub add_events: HashSet<String>,
    pub init_resources: HashSet<String>,
    pub insert_resources: HashSet<InsertResourceSerializedArgsWithPath>,
    pub init_states: HashSet<String>,
    pub auto_names: HashSet<String>,
    pub add_systems: HashSet<AddSystemSerializedArgs>,
}

pub fn default_app_ident() -> Ident {
    Ident::new("app", Span::call_site())
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn __wasm_call_ctors();
}

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}
