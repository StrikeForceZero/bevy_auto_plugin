use crate::__private::attribute_args::GenericsArgs;
use crate::__private::type_list::TypeList;
use proc_macro2::Span;
use quote::ToTokens;
use syn::Path;

pub trait HasGenericsCollection {
    type CollectionItem: ToTokens;
    type Collection: IntoIterator<Item = Self::CollectionItem>;
    fn generics(&self) -> syn::Result<Self::Collection>;
}

impl HasGenericsCollection for Path {
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
    fn count_generics(&self) -> syn::Result<usize>;
}

impl CountGenerics for Path {
    fn get_span(&self) -> Span {
        syn::spanned::Spanned::span(&self)
    }

    fn count_generics(&self) -> syn::Result<usize> {
        crate::__private::util::extensions::path::PathExt::generic_count(self)
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

    fn count_generics(&self) -> syn::Result<usize> {
        let iter = self
            .generics()
            .into_iter()
            .map(|g| g.count_generics())
            .collect::<syn::Result<Vec<_>>>()?;
        let &max = iter.iter().max().unwrap_or(&0);
        let &min = iter.iter().min().unwrap_or(&0);
        if max != min {
            return Err(syn::Error::new(
                self.get_span(),
                format!("inconsistent number of generics specified min: {min}, max: {max}"),
            ));
        }
        Ok(max)
    }
}
