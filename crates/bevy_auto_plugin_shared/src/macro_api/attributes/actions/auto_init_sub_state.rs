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
pub type QInitSubState = AppMutationEmitter<IaInitSubState>;
pub type QQInitSubState = QQ<IaInitSubState>;

impl EmitAppMutationTokens for QInitSubState {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.add_sub_state::<#target>();
        });
    }
}

impl ToTokens for QQInitSubState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
