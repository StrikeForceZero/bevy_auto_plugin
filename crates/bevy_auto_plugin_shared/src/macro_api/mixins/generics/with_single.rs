use crate::macro_api::mixins::HasKeys;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(Debug, Clone, Default, FromMeta, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct WithZeroOrOneGenerics {
    #[darling(default, rename = "generics")]
    pub generics: Option<TypeList>,
}

impl WithZeroOrOneGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithZeroOrOneGenerics {
    fn keys() -> &'static [&'static str] {
        WithZeroOrOneGenerics::KEYS
    }
}

impl HasGenerics for WithZeroOrOneGenerics {
    fn generics(&self) -> &[TypeList] {
        self.generics.as_slice()
    }
}

impl ToTokens for WithZeroOrOneGenerics {
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
        WithZeroOrOneGenerics { generics: None }
            .to_token_stream()
            .to_string()
            == r#""#;
    }

    #[xtest]
    fn test_to_tokens_single() {
        WithZeroOrOneGenerics {
            generics: Some(TypeList(vec![parse_quote!(bool), parse_quote!(u32)])),
        }
        .to_token_stream()
        .to_string()
            == r#"generics(bool, u32)"#;
    }
}
