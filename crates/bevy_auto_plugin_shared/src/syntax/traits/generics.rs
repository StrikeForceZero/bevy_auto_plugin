use crate::macro_api::with_plugin::GenericsArgs;
use proc_macro2::Span;
use quote::ToTokens;

pub trait HasGenericsCollection {
    type CollectionItem: ToTokens;
    type Collection: IntoIterator<Item = Self::CollectionItem>;
    fn generics(&self) -> syn::Result<Self::Collection>;
}

pub trait CountGenerics {
    fn get_span(&self) -> Span;
    fn count_generics(&self) -> syn::Result<usize>;
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
