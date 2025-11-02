use macro_paths::MacroPaths;

mod macro_paths;

// TODO: pass from proc macros and resolve any aliases - instead of defaulting
#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Context {
    pub macros: MacroPaths,
}

pub mod prelude {
    pub use super::{
        Context,
        macro_paths::*,
    };
}
