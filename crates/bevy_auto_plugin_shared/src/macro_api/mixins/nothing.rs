use crate::{
    macro_api::mixins::{
        HasKeys,
        generics::HasGenerics,
    },
    syntax::ast::type_list::TypeList,
};
use darling::FromMeta;

#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
pub struct Nothing {}

impl HasGenerics for Nothing {
    fn generics(&self) -> &[TypeList] {
        &[]
    }
}

impl HasKeys for Nothing {
    fn keys() -> &'static [&'static str] {
        &[]
    }
}
