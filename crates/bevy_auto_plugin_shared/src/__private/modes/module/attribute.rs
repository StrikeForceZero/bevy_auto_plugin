use darling::FromMeta;
use proc_macro2::{Ident, Span};
use smart_default::SmartDefault;

/// Parsed contents of `#[module_auto_plugin(init_name = ...)]`
#[derive(Debug, SmartDefault, FromMeta)]
#[darling(default)]
pub struct ModuleArgs {
    #[darling(rename = "init_name")]
    #[default(Ident::new("init", Span::call_site()))]
    pub init_name: Ident,
}
