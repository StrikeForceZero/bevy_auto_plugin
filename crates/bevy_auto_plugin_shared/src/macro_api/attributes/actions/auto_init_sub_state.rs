use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitSubStateArgs {}

impl AttributeIdent for InitSubStateArgs {
    const IDENT: &'static str = "auto_init_sub_state";
}

pub type IaInitSubState =
    ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type QInitSubState<'a> = Q<'a, IaInitSubState>;
pub type QQInitSubState<'a> = QQ<'a, IaInitSubState>;

impl ToTokensWithAppParam for QInitSubState<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.init_sub_state::<#target>();
        });
    }
}

impl ToTokens for QQInitSubState<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
