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

pub type IaRunOnBuild =
    ItemAttribute<Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>, AllowStructOrEnum>;
pub type QRunOnBuild<'a> = Q<'a, IaRunOnBuild>;
pub type QQRunOnBuild<'a> = QQ<'a, IaRunOnBuild>;

impl RequiredUseQTokens for QRunOnBuild<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident) {
        for concrete_path in self.args.concrete_paths() {
            tokens.extend(quote! {
                #concrete_path(#app_param);
            });
        }
    }
}

impl ToTokens for QQRunOnBuild<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
