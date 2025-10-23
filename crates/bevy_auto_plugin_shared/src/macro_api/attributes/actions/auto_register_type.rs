use crate::macro_api::attributes::{AllowStructOrEnum, AttributeIdent, GenericsCap, ItemAttribute};
use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterTypeArgs {}

impl AttributeIdent for RegisterTypeArgs {
    const IDENT: &'static str = "auto_register_type";
}

pub type RegisterType = ItemAttribute<
    Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QRegisterTypeArgs<'a> = Q<'a, RegisterType>;
pub type QQRegisterTypeArgs<'a> = QQ<'a, RegisterType>;

impl RequiredUseQTokens for QRegisterTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #app_param.register_type::<#concrete_path>();
            });
        }
    }
}
impl ToTokens for QQRegisterTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
