use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct NameArgs {
    pub name: Option<syn::Lit>,
}

impl AttributeIdent for NameArgs {
    const IDENT: &'static str = "auto_name";
}
