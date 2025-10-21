use crate::macro_api::attributes::AttributeIdent;
use crate::syntax::ast::flag_or_expr::FlagOrExpr;
use darling::FromMeta;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddPluginArgs {
    #[darling(default)]
    pub init: FlagOrExpr,
}

impl AttributeIdent for AddPluginArgs {
    const IDENT: &'static str = "auto_add_plugin";
}
