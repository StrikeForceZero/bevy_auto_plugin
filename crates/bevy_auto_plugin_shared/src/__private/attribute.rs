use crate::codegen::ExpandAttrs;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use proc_macro2::TokenStream as MacroStream;

pub trait RewriteAttribute {
    fn expand_args(&self, plugin: &NonEmptyPath) -> MacroStream;
    fn expand_attrs(&self, plugin: &NonEmptyPath) -> ExpandAttrs;
}
