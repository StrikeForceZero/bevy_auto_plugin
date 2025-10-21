use crate::macro_api::macro_paths::MacroPaths;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub macros: MacroPaths,
}
