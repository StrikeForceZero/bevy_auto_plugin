use crate::__private::attribute_args::{AddSystemArgs, InsertResourceArgsWithPath};
use syn::Path;

#[derive(Debug, Clone, Copy)]
pub enum TargetRequirePath {
    RegisterTypes,
    RegisterStateTypes,
    AddEvents,
    InitResources,
    InitStates,
    RequiredComponentAutoName,
    AddObserver,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TargetData {
    RegisterTypes(Path),
    RegisterStateTypes(Path),
    AddEvents(Path),
    InitResources(Path),
    InitStates(Path),
    RequiredComponentAutoName(Path),
    InsertResource(InsertResourceArgsWithPath),
    AddSystem { system: Path, params: AddSystemArgs },
    AddObserver(Path),
}

impl TargetData {
    pub fn from_target_require_path(target_require_path: TargetRequirePath, path: Path) -> Self {
        match target_require_path {
            TargetRequirePath::RegisterTypes => Self::RegisterTypes(path),
            TargetRequirePath::RegisterStateTypes => Self::RegisterStateTypes(path),
            TargetRequirePath::AddEvents => Self::AddEvents(path),
            TargetRequirePath::InitResources => Self::InitResources(path),
            TargetRequirePath::InitStates => Self::InitStates(path),
            TargetRequirePath::RequiredComponentAutoName => Self::RequiredComponentAutoName(path),
            TargetRequirePath::AddObserver => Self::AddObserver(path),
        }
    }
}
