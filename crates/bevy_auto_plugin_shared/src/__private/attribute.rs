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
