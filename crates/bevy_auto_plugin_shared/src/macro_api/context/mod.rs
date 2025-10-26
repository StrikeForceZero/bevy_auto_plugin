use macro_paths::MacroPaths;

pub mod macro_paths;

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Context {
    pub macros: MacroPaths,
}
