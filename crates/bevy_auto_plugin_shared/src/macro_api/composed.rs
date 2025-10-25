use crate::macro_api::attributes::{AttributeIdent, ItemAttributeArgs};
use crate::macro_api::context::Context;
use crate::macro_api::macro_paths::MacroPathProvider;
use crate::macro_api::mixins::Mixin;
use crate::macro_api::mixins::generics::HasGenerics;
use crate::macro_api::mixins::nothing::Nothing;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::ast::type_list::TypeList;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use syn::parse::{Parse, ParseStream};
use syn::parse_quote;
use syn::punctuated::Punctuated;

#[derive(Debug, Clone, PartialEq)]
pub struct Composed<CBase, MPlugin = Nothing, MGenerics = Nothing> {
    pub base: CBase,
    pub plugin: MPlugin,
    pub generics: MGenerics,
}

impl<C, P, G> Hash for Composed<C, P, G>
where
    C: Hash,
    P: Hash,
    G: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.plugin.hash(state);
        self.generics.hash(state);
    }
}

impl<CBase, MPlugin, MGenerics> FromMeta for Composed<CBase, MPlugin, MGenerics>
where
    CBase: FromMeta,
    MPlugin: Mixin,
    MGenerics: Mixin,
{
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let plugin_keys: HashSet<&str> = MPlugin::keys().iter().copied().collect();
        let generics_keys: HashSet<&str> = MGenerics::keys().iter().copied().collect();

        let mut plugin_bucket = Vec::<NestedMeta>::new();
        let mut generics_bucket = Vec::<NestedMeta>::new();
        let mut base_bucket = Vec::<NestedMeta>::new();

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
                if plugin_keys.contains(k.as_str()) {
                    plugin_bucket.push(nm.clone());
                    true
                } else if generics_keys.contains(k.as_str()) {
                    generics_bucket.push(nm.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if !routed {
                base_bucket.push(nm.clone());
            }
        }

        // Parse each bucket
        let base = CBase::from_list(&base_bucket)?;
        let plugin = MPlugin::from_list(&plugin_bucket)?;
        let generics = MGenerics::from_list(&generics_bucket)?;

        Ok(Self {
            base,
            plugin,
            generics,
        })
    }
}

impl<CBase, MPlugin, MGenerics> Parse for Composed<CBase, MPlugin, MGenerics>
where
    CBase: FromMeta,
    MPlugin: Mixin,
    MGenerics: Mixin,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse `#[attr(<here>)]`'s inner as: key = value, key(...), literal, ...
        let list: Punctuated<NestedMeta, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let items: Vec<NestedMeta> = list.into_iter().collect();
        Composed::<CBase, MPlugin, MGenerics>::from_list(&items)
            .map_err(|e| syn::Error::new(e.span(), e.to_string()))
    }
}

impl<T, P, G> AttributeIdent for Composed<T, P, G>
where
    T: AttributeIdent,
{
    const IDENT: &'static str = T::IDENT;
}

impl<T, P, G> ItemAttributeArgs for Composed<T, P, G>
where
    T: ItemAttributeArgs,
    P: Clone,
    G: Clone,
{
}

impl<CBase, MGenerics> Composed<CBase, WithPlugin, MGenerics> {
    pub fn plugin(&self) -> &syn::Path {
        &self.plugin.plugin
    }
}

impl<CBase, MPlugin, MGenerics> Composed<CBase, MPlugin, MGenerics>
where
    MGenerics: HasGenerics,
{
    pub fn generics(&self) -> &[TypeList] {
        self.generics.generics()
    }
    #[allow(dead_code)]
    // TODO: which one to use?
    pub fn concrete_paths(&self, target: &syn::Path) -> Vec<syn::Path> {
        if self.generics.generics().is_empty() {
            vec![target.clone()]
        } else {
            self.generics
                .generics()
                .iter()
                .map(|g| parse_quote!(#target :: < #g >))
                .collect()
        }
    }
}

impl<CBase, MPlugin, MGenerics> Composed<CBase, MPlugin, MGenerics>
where
    MPlugin: ToTokens,
    MGenerics: ToTokens,
{
    pub fn extra_args(&self) -> Vec<TokenStream> {
        let plugin_tokens = self.plugin.to_token_stream();
        let generics_tokens = self.generics.to_token_stream();
        match (plugin_tokens.is_empty(), generics_tokens.is_empty()) {
            (true, true) => vec![],
            (false, true) => vec![plugin_tokens],
            (true, false) => vec![generics_tokens],
            (false, false) => vec![plugin_tokens, generics_tokens],
        }
    }
}

impl<CBase, MPlugin, MGenerics> MacroPathProvider for Composed<CBase, MPlugin, MGenerics>
where
    CBase: MacroPathProvider,
{
    fn macro_path(context: &Context) -> &syn::Path {
        CBase::macro_path(context)
    }
}
