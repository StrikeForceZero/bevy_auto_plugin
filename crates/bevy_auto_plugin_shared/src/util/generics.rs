use crate::attribute_args::{InsertResourceArgs, StructOrEnumAttributeArgs};
use crate::type_list::TypeList;
use crate::util::meta::struct_or_enum_meta::StructOrEnumMeta;
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
        Ok(vec![crate::util::extensions::path::PathExt::generics(
            self,
        )?])
    }
}

pub trait CountGenerics: HasGenericCollection {
    fn get_span(&self) -> Span;
    // TODO: rename to count_generics
    fn count_generics(&self) -> usize;
}

impl CountGenerics for Path {
    fn get_span(&self) -> Span {
        syn::spanned::Spanned::span(&self)
    }

    fn count_generics(&self) -> usize {
        crate::util::extensions::path::PathExt::generic_count(self).unwrap_or(0)
    }
}

impl HasGenericCollection for StructOrEnumAttributeArgs {
    type CollectionItem = TypeList;
    type Collection = Vec<TypeList>;

    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(self.generics.clone())
    }
}

impl CountGenerics for StructOrEnumAttributeArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics
            .first()
            .map(|g| g.span())
            .unwrap_or(Span::call_site())
    }

    fn count_generics(&self) -> usize {
        let iter = self.generics.iter().map(|g| g.len()).collect::<Vec<_>>();
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

impl HasGenericCollection for InsertResourceArgs {
    type CollectionItem = TypeList;
    type Collection = Vec<TypeList>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        let Some(generics) = self.generics.clone() else {
            return Ok(vec![]);
        };
        Ok(vec![generics])
    }
}

impl CountGenerics for InsertResourceArgs {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count_generics(&self) -> usize {
        let Some(generics) = &self.generics else {
            return 0;
        };
        generics.len()
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
