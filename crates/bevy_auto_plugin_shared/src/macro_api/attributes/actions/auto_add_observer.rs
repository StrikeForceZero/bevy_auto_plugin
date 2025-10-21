use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddObserverArgs {}

impl AttributeIdent for AddObserverArgs {
    const IDENT: &'static str = "auto_add_observer";
}
