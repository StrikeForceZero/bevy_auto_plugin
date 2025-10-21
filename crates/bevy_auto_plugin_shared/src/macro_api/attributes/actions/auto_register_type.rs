use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterTypeArgs {}

impl AttributeIdent for RegisterTypeArgs {
    const IDENT: &'static str = "auto_register_type";
}
