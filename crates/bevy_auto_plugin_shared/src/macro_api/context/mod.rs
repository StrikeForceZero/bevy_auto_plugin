use macro_paths::MacroPaths;

mod macro_paths;

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
