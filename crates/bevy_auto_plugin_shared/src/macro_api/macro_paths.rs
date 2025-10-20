use crate::macro_api::attributes::prelude::AddSystemArgs;
use crate::macro_api::context::Context;

pub struct MacroPaths {
    /// resolved absolute path to `auto_add_system`
    pub emit_add_system_macro: syn::Path,
    // .. others
}

pub trait MacroPathProvider {
    fn macro_path(context: &Context) -> &syn::Path;
}

impl MacroPathProvider for AddSystemArgs {
    fn macro_path(context: &Context) -> &syn::Path {
        &context.macros.emit_add_system_macro
    }
}
