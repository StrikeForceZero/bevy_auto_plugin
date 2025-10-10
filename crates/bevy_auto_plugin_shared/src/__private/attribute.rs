use crate::codegen::ExpandAttrs;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use proc_macro2::TokenStream as MacroStream;

pub trait AutoPluginAttribute {
    fn ident_str(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginItemAttribute {
    RegisterType,
    AddMessage,
    InitResource,
    InsertResource,
    InitState,
    AutoName,
    RegisterStateType,
    AddSystem,
    AddObserver,
}

impl AutoPluginItemAttribute {
    pub const fn ident_str(&self) -> &'static str {
        match self {
            Self::RegisterType => "auto_register_type",
            Self::AddMessage => "auto_add_message",
            Self::InitResource => "auto_init_resource",
            Self::InsertResource => "auto_insert_resource",
            Self::InitState => "auto_init_state",
            Self::AutoName => "auto_name",
            Self::RegisterStateType => "auto_register_state_type",
            Self::AddSystem => "auto_add_system",
            Self::AddObserver => "auto_add_observer",
        }
    }
}

impl AutoPluginAttribute for AutoPluginItemAttribute {
    fn ident_str(&self) -> &'static str {
        Self::ident_str(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginShortHandAttribute {
    Component,
    Resource,
    Event,
    Message,
    States,
    System,
    Observer,
}

impl AutoPluginShortHandAttribute {
    pub const fn ident_str(&self) -> &'static str {
        match self {
            Self::Component => "auto_component",
            Self::Resource => "auto_resource",
            Self::Event => "auto_event",
            Self::Message => "auto_message",
            Self::States => "auto_states",
            Self::System => "auto_system",
            Self::Observer => "auto_observer",
        }
    }
}

impl AutoPluginAttribute for AutoPluginShortHandAttribute {
    fn ident_str(&self) -> &'static str {
        Self::ident_str(self)
    }
}

pub trait ShortHandAttribute {
    fn expand_args(&self, plugin: &NonEmptyPath) -> MacroStream;
    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs;
}
