use crate::macro_api::attributes::{AllowStructOrEnum, AttributeIdent, GenericsCap, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::prelude::{WithPlugin, WithZeroOrManyGenerics};
use crate::macro_api::q::{Q, RequiredUseQTokens};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitResourceArgs {}

impl AttributeIdent for InitResourceArgs {
    const IDENT: &'static str = "auto_init_resource";
}

pub type InitResource = ItemAttribute<
    Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QInitResourceArgs<'a> = Q<'a, InitResource>;

impl RequiredUseQTokens for QInitResourceArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.init_resource::<#concrete_path>();
            });
        }
    }
}
