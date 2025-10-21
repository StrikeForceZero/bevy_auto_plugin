use crate::macro_api::attributes::{AllowStructOrEnum, AttributeIdent, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::prelude::{WithNoGenerics, WithPlugin};
use crate::macro_api::q::{Q, RequiredUseQTokens};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitStateArgs {}

impl AttributeIdent for InitStateArgs {
    const IDENT: &'static str = "auto_init_state";
}

pub type InitState =
    ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type QInitStateArgs<'a> = Q<'a, InitState>;

impl RequiredUseQTokens for QInitStateArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.init_state::<#target>();
        });
    }
}
