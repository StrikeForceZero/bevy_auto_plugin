use crate::macro_api::attributes::AttributeIdent;
use crate::syntax::ast::any_expr::AnyExprCallClosureMacroPath;
use darling::FromMeta;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct InsertResourceArgs {
    pub resource: AnyExprCallClosureMacroPath,
}

impl AttributeIdent for InsertResourceArgs {
    const IDENT: &'static str = "auto_insert_resource";
}
