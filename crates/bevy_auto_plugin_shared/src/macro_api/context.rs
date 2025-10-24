use crate::macro_api::macro_paths::MacroPaths;

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Context {
    pub macros: MacroPaths,
}
