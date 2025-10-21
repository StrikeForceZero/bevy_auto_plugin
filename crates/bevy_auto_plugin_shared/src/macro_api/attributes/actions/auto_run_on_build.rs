use crate::macro_api::attributes::AttributeIdent;
use darling::FromMeta;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct RunOnBuildArgs {}

impl AttributeIdent for RunOnBuildArgs {
    const IDENT: &'static str = "auto_run_on_build";
}
