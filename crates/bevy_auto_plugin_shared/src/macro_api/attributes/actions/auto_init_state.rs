use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitStateArgs {}

impl AttributeIdent for InitStateArgs {
    const IDENT: &'static str = "auto_init_state";
}

pub type IaInitState =
    ItemAttribute<Composed<InitStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type QInitState = AppMutationEmitter<IaInitState>;
pub type QQInitState = QQ<IaInitState>;

impl EmitAppMutationTokens for QInitState {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.init_state::<#target>();
        });
    }
}

impl ToTokens for QQInitState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
