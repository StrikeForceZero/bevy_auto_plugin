use crate::macro_api::context::Context;
use crate::macro_api::macro_paths::MacroPathProvider;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::macro_api::mixins::nothing::Nothing;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::macro_api::mixins::{HasKeys, Mixin};
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::parse_quote;
use syn::punctuated::Punctuated;

#[derive(Debug)]
pub struct Composed<C, M1 = Nothing, M2 = Nothing> {
    pub core: C,
    // todo: rename to plugin?
    pub m1: M1,
    // todo: rename to generics?
    pub m2: M2,
}

impl<C, M1, M2> FromMeta for Composed<C, M1, M2>
where
    C: FromMeta,
    M1: Mixin,
    M2: Mixin,
{
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let keys1: HashSet<&str> = M1::keys().iter().copied().collect();
        let keys2: HashSet<&str> = M2::keys().iter().copied().collect();

        let mut bucket1 = Vec::<NestedMeta>::new();
        let mut bucket2 = Vec::<NestedMeta>::new();
        let mut bucket_core = Vec::<NestedMeta>::new();

        for nm in items {
            let key_opt = match &nm {
                NestedMeta::Meta(syn::Meta::Path(p)) => {
                    p.segments.last().map(|s| s.ident.to_string())
                }
                NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    nv.path.segments.last().map(|s| s.ident.to_string())
                }
                NestedMeta::Meta(syn::Meta::List(ml)) => {
                    ml.path.segments.last().map(|s| s.ident.to_string())
                }
                NestedMeta::Lit(_) => None,
            };

            let routed = if let Some(ref k) = key_opt {
                if keys1.contains(k.as_str()) {
                    bucket1.push(nm.clone());
                    true
                } else if keys2.contains(k.as_str()) {
                    bucket2.push(nm.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if !routed {
                bucket_core.push(nm.clone());
            }
        }

        // Parse each bucket
        let core = C::from_list(&bucket_core)?;
        let m1 = M1::from_list(&bucket1)?;
        let m2 = M2::from_list(&bucket2)?;

        Ok(Self { core, m1, m2 })
    }
}

impl<C, M1, M2> Parse for Composed<C, M1, M2>
where
    C: FromMeta,
    M1: Mixin,
    M2: Mixin,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse `#[attr(<here>)]`'s inner as: key = value, key(...), literal, ...
        let list: Punctuated<NestedMeta, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let items: Vec<NestedMeta> = list.into_iter().collect();
        Composed::<C, M1, M2>::from_list(&items)
            .map_err(|e| syn::Error::new(e.span(), e.to_string()))
    }
}

impl<C, M1, M2> Composed<C, M1, M2> {
    pub fn args(&self) -> &C {
        &self.core
    }
}

impl<C, M2> Composed<C, WithPlugin, M2> {
    pub fn plugin(&self) -> &syn::Path {
        &self.m1.plugin
    }
}

impl<C, M1, M2> Composed<C, M1, M2>
where
    M2: HasGenerics,
{
    pub fn generics(&self) -> &[TypeList] {
        self.m2.generics()
    }
    pub fn concrete_paths(&self, target: &syn::Path) -> Vec<syn::Path> {
        if self.m2.generics().is_empty() {
            vec![target.clone()]
        } else {
            self.m2
                .generics()
                .iter()
                .map(|g| parse_quote!(#target :: < #g >))
                .collect()
        }
    }
}

impl<C, M1, M2> Composed<C, M1, M2>
where
    M1: ToTokens,
    M2: ToTokens,
{
    pub fn extra_args(&self) -> Vec<TokenStream> {
        let m1_tokens = self.m1.to_token_stream();
        let m2_tokens = self.m2.to_token_stream();
        match (m1_tokens.is_empty(), m2_tokens.is_empty()) {
            (true, true) => vec![],
            (false, true) => vec![m1_tokens],
            (true, false) => vec![m2_tokens],
            (false, false) => vec![m1_tokens, m2_tokens],
        }
    }
}

impl<C, M1, M2> MacroPathProvider for Composed<C, M1, M2>
where
    C: MacroPathProvider,
{
    fn macro_path(context: &Context) -> &syn::Path {
        C::macro_path(context)
    }
}
