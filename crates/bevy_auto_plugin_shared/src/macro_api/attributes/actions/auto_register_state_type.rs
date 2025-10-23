use crate::macro_api::attributes::{AllowStructOrEnum, AttributeIdent, GenericsCap, ItemAttribute};
use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterStateTypeArgs {}

impl AttributeIdent for RegisterStateTypeArgs {
    const IDENT: &'static str = "auto_register_state_type";
}

pub type RegisterStateType = ItemAttribute<
    Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QRegisterStateTypeArgs<'a> = Q<'a, RegisterStateType>;
pub type QQRegisterStateTypeArgs<'a> = QQ<'a, RegisterStateType>;

impl RequiredUseQTokens for QRegisterStateTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            let bevy_state = crate::__private::paths::state::root_path();
            tokens.extend(quote! {
                #app_param.register_type :: < #bevy_state::prelude::State< #concrete_path > >();
                #app_param.register_type :: < #bevy_state::prelude::NextState< #concrete_path > >();
            });
        }
    }
}

impl ToTokens for QQRegisterStateTypeArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
