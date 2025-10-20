use crate::macro_api::mixins::HasKeys;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

#[derive(Debug, Clone, Default)]
pub struct WithoutGenerics {}

impl WithoutGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithoutGenerics {
    fn keys() -> &'static [&'static str] {
        WithoutGenerics::KEYS
    }
}

impl HasGenerics for WithoutGenerics {
    fn generics(&self) -> &[TypeList] {
        &[]
    }
}

impl ToTokens for WithoutGenerics {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}

impl FromMeta for WithoutGenerics {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if !items.is_empty() {
            let errors = items
                .iter()
                .map(|item| {
                    darling::Error::unsupported_shape("generics are not supported")
                        .with_span(&item.span())
                })
                .collect();
            return Err(darling::Error::multiple(errors));
        }
        Ok(Self::default())
    }
}

impl Parse for WithoutGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            return Err(input.error("generics are not supported"));
        }
        Ok(Self::default())
    }
}
