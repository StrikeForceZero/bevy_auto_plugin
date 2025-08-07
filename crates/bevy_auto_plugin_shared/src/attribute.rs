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
