use crate::macro_api::emitters::app_mutation::{AppMutationEmitter, EmitAppMutationTokens};
use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitResourceArgs {}

impl AttributeIdent for InitResourceArgs {
    const IDENT: &'static str = "auto_init_resource";
}

pub type IaInitResource = ItemAttribute<
    Composed<InitResourceArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type InitResourceAppMutEmitter = AppMutationEmitter<IaInitResource>;
pub type InitResourceAttrEmitter = AttrEmitter<IaInitResource>;

impl EmitAppMutationTokens for InitResourceAppMutEmitter {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.init_resource::<#concrete_path>();
            });
        }
    }
}

impl ToTokens for InitResourceAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
