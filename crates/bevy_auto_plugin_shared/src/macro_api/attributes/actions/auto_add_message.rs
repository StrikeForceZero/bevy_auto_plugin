use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddMessageArgs {}

impl AttributeIdent for AddMessageArgs {
    const IDENT: &'static str = "auto_add_message";
}
