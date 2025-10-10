use crate::syntax::ast::type_list::TypeList;
use crate::syntax::extensions::path::PathExt;
use crate::syntax::traits::generics::{CountGenerics, HasGenericsCollection};
use syn::spanned::Spanned;

pub struct PathGenerics<'a>(pub &'a syn::Path);

impl HasGenericsCollection for PathGenerics<'_> {
    type CollectionItem = TypeList;
    type Collection = Vec<TypeList>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![PathExt::generics(self.0)?])
    }
}

impl CountGenerics for PathGenerics<'_> {
    fn get_span(&self) -> proc_macro2::Span {
        self.0.span()
    }
    fn count_generics(&self) -> syn::Result<usize> {
        PathExt::generic_count(self.0)
    }
}
