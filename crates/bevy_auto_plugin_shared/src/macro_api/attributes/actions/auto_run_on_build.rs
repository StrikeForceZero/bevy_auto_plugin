use crate::macro_api::attributes::{AllowStructOrEnum, AttributeIdent, GenericsCap, ItemAttribute};
use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct RunOnBuildArgs {}

impl AttributeIdent for RunOnBuildArgs {
    const IDENT: &'static str = "auto_run_on_build";
}

pub type RunOnBuild =
    ItemAttribute<Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QRunOnBuildArgs<'a> = Q<'a, RunOnBuild>;
pub type QQRunOnBuildArgs<'a> = QQ<'a, RunOnBuild>;

impl RequiredUseQTokens for QRunOnBuildArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #concrete_path(#app_param);
            });
        }
    }
}

impl ToTokens for QQRunOnBuildArgs<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
