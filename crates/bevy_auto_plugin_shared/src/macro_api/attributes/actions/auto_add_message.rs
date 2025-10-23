use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddMessageArgs {}

impl AttributeIdent for AddMessageArgs {
    const IDENT: &'static str = "auto_add_message";
}

pub type IaAddMessage =
    ItemAttribute<Composed<AddMessageArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QAddMessage<'a> = Q<'a, IaAddMessage>;
pub type QQAddMessage<'a> = QQ<'a, IaAddMessage>;

impl RequiredUseQTokens for QAddMessage<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.add_message::<#concrete_path>();
            });
        }
    }
}

impl ToTokens for QQAddMessage<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
