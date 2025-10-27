use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitSubStateArgs {}

impl AttributeIdent for InitSubStateArgs {
    const IDENT: &'static str = "auto_init_sub_state";
}

pub type IaInitSubState =
    ItemAttribute<Composed<InitSubStateArgs, WithPlugin, WithNoGenerics>, AllowStructOrEnum>;
pub type InitSubStateAppMutEmitter = AppMutationEmitter<IaInitSubState>;
pub type InitSubStateAttrEmitter = AttrEmitter<IaInitSubState>;

impl EmitAppMutationTokens for InitSubStateAppMutEmitter {
    fn to_app_mutation_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        let target = &self.args.target;
        tokens.extend(quote! {
            #app_param.add_sub_state::<#target>();
        });
    }
}

impl ToTokens for InitSubStateAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
