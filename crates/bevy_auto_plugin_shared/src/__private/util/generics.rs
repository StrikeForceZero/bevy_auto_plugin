use crate::__private::attribute_args::GenericsArgs;
use crate::__private::type_list::TypeList;
use crate::__private::util::meta::struct_or_enum_meta::StructOrEnumMeta;
use proc_macro2::Span;
use quote::ToTokens;
use syn::Path;

pub trait HasGenericCollection {
    type CollectionItem: ToTokens;
    type Collection: IntoIterator<Item = Self::CollectionItem>;
    fn generics(&self) -> syn::Result<Self::Collection>;
}

impl HasGenericCollection for Path {
    type CollectionItem = TypeList;
    type Collection = Vec<TypeList>;

    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![
            crate::__private::util::extensions::path::PathExt::generics(self)?,
        ])
    }
}

pub trait CountGenerics {
    fn get_span(&self) -> Span;
    // TODO: rename to count_generics
    fn count_generics(&self) -> usize;
}

impl CountGenerics for Path {
    fn get_span(&self) -> Span {
        syn::spanned::Spanned::span(&self)
    }

    fn count_generics(&self) -> usize {
        crate::__private::util::extensions::path::PathExt::generic_count(self).unwrap_or(0)
    }
}

impl<T> CountGenerics for T
where
    T: GenericsArgs,
{
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.type_lists()
            .first()
            .map(|g| g.span())
            .unwrap_or(Span::call_site())
    }

    fn count_generics(&self) -> usize {
        let iter = self
            .generics()
            .into_iter()
            .map(|g| g.count_generics())
            .collect::<Vec<_>>();
        let &max = iter.iter().max().unwrap_or(&0);
        let &min = iter.iter().min().unwrap_or(&0);
        // TODO: return result
        assert_eq!(
            max, min,
            "inconsistent number of generics specified min: {min}, max: {max}"
        );
        max
    }
}

impl HasGenericCollection for TypeList {
    type CollectionItem = Self;
    type Collection = Vec<Self>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![self.clone()])
    }
}

impl CountGenerics for TypeList {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.span()
    }

    fn count_generics(&self) -> usize {
        self.len()
    }
}

impl<'a> HasGenericCollection for StructOrEnumMeta<'a> {
    type CollectionItem = &'a syn::Generics;
    type Collection = Vec<&'a syn::Generics>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![self.generics])
    }
}

impl CountGenerics for StructOrEnumMeta<'_> {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count_generics(&self) -> usize {
        self.generics.params.len()
    }
}
