use crate::__private::type_list::TypeList;
use crate::__private::util::generics_traits::CountGenerics;
use proc_macro2::{Span, TokenStream as MacroStream};
use quote::{ToTokens, quote};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct GenericsCollection(pub Vec<TypeList>);

impl GenericsCollection {
    pub fn iter_with_default_generics_when_empty(self) -> impl IntoIterator<Item = Generics> {
        let mut vec = self.0.into_iter().map(Generics).collect::<Vec<_>>();

        if vec.is_empty() {
            vec.push(Generics::default());
        }

        vec.into_iter()
    }
}

impl IntoIterator for GenericsCollection {
    type Item = Generics;
    type IntoIter = std::vec::IntoIter<Generics>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(Generics)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Generics(pub TypeList);

impl Generics {
    pub fn empty() -> Self {
        Generics(TypeList::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        if self.is_empty() {
            return;
        }
        let types = &self.0;
        tokens.extend(quote!(< #types >));
    }
}

impl CountGenerics for Generics {
    fn get_span(&self) -> Span {
        self.0.get_span()
    }

    fn count_generics(&self) -> syn::Result<usize> {
        Ok(self.0.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromMeta;
    use quote::quote;
    use syn::Type;
    use syn::TypePath;

    fn types() -> syn::Result<TypeList> {
        let ty_u32 = Type::Path(TypePath::from_string("u32")?);
        let ty_bool = Type::Path(TypePath::from_string("bool")?);
        Ok(TypeList(vec![ty_u32, ty_bool]))
    }

    #[internal_test_proc_macro::xtest]
    fn test_generics() -> syn::Result<()> {
        assert_eq!(
            Generics(TypeList::empty()).to_token_stream().to_string(),
            quote!().to_string()
        );
        assert_eq!(
            Generics(types()?).to_token_stream().to_string(),
            quote!(<u32, bool>).to_string()
        );
        assert_eq!(
            Generics(types()?).to_token_stream().to_string(),
            quote!(<u32, bool>).to_string()
        );
        Ok(())
    }

    #[internal_test_proc_macro::xtest]
    fn test_generics_collection() -> syn::Result<()> {
        let generics = GenericsCollection(vec![types()?]);
        let mut iter = generics.into_iter();
        assert_eq!(iter.next(), Some(Generics(types()?)));
        assert_eq!(iter.next(), None);
        Ok(())
    }
}
