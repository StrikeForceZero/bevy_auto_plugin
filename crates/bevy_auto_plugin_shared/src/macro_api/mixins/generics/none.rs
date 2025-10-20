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
pub struct WithNoGenerics {}

impl WithNoGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithNoGenerics {
    fn keys() -> &'static [&'static str] {
        WithNoGenerics::KEYS
    }
}

impl HasGenerics for WithNoGenerics {
    fn generics(&self) -> &[TypeList] {
        &[]
    }
}

impl ToTokens for WithNoGenerics {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}

impl FromMeta for WithNoGenerics {
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

impl Parse for WithNoGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            return Err(input.error("generics are not supported"));
        }
        Ok(Self::default())
    }
}
