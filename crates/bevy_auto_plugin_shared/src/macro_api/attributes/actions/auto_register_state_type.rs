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

pub type IaRegisterStateType = ItemAttribute<
    Composed<RegisterStateTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type QRegisterStateType<'a> = Q<'a, IaRegisterStateType>;
pub type QQRegisterStateType<'a> = QQ<'a, IaRegisterStateType>;

impl ToTokensWithAppParam for QRegisterStateType<'_> {
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

impl ToTokens for QQRegisterStateType<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
