use crate::{
    macro_api::mixins::{
        HasKeys,
        generics::HasGenerics,
    },
    syntax::ast::type_list::TypeList,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(Debug, Clone, Default, FromMeta, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct WithZeroOrManyGenerics {
    #[darling(multiple, default, rename = "generics")]
    pub generics: Vec<TypeList>,
}

impl WithZeroOrManyGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithZeroOrManyGenerics {
    fn keys() -> &'static [&'static str] {
        WithZeroOrManyGenerics::KEYS
    }
}

impl HasGenerics for WithZeroOrManyGenerics {
    fn generics(&self) -> &[TypeList] {
        &self.generics
    }
}

impl ToTokens for WithZeroOrManyGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sets = self.generics.iter().map(|g| quote! { generics(#g) });
        tokens.extend(quote! {
            #(#sets),*
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::parse_quote;

    #[xtest]
    fn test_to_tokens_zero() {
        assert_eq!(
            WithZeroOrManyGenerics { generics: vec![] }.to_token_stream().to_string(),
            quote!().to_string()
        );
    }

    #[xtest]
    fn test_to_tokens_single() {
        assert_eq!(
            WithZeroOrManyGenerics {
                generics: vec![TypeList::from_types(vec![
                    parse_quote!(bool),
                    parse_quote!(u32)
                ])]
            }
            .to_token_stream()
            .to_string(),
            quote!(generics(bool, u32)).to_string()
        );
    }

    #[xtest]
    fn test_to_tokens_multiple() {
        assert_eq!(
            WithZeroOrManyGenerics {
                generics: vec![
                    TypeList::from_types(vec![parse_quote!(bool), parse_quote!(u32)]),
                    TypeList::from_types(vec![parse_quote!(usize)]),
                ],
            }
            .to_token_stream()
            .to_string(),
            quote!(generics(bool, u32), generics(usize)).to_string()
        );
    }
}
