use crate::{
    macro_api::{
        mixins::HasKeys,
        prelude::{
            WithZeroOrManyGenerics,
            WithZeroOrOneGenerics,
        },
    },
    util::macros::impl_from_default,
};
use darling::{
    FromMeta,
    ast::NestedMeta,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    spanned::Spanned,
};

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct WithNoGenerics {}

impl WithNoGenerics {
    pub const KEYS: &'static [&'static str] = &["generics"];
}

impl HasKeys for WithNoGenerics {
    fn keys() -> &'static [&'static str] {
        WithNoGenerics::KEYS
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

impl_from_default!(WithNoGenerics => (WithZeroOrOneGenerics, WithZeroOrManyGenerics));
