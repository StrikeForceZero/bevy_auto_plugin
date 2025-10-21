use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitStateArgs {}

impl AttributeIdent for InitStateArgs {
    const IDENT: &'static str = "auto_init_state";
}
