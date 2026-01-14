use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct RunOnBuildArgs {}

impl AttributeIdent for RunOnBuildArgs {
    const IDENT: &'static str = "auto_run_on_build";
}

pub type IaRunOnBuild =
    ItemAttribute<Composed<RunOnBuildArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type RunOnBuildAppMutEmitter = AppMutationEmitter<IaRunOnBuild>;
pub type RunOnBuildAttrEmitter = AttrEmitter<IaRunOnBuild>;

impl EmitAppMutationTokens for RunOnBuildAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #concrete_path(#app_param);
            });
        }
        Ok(())
    }
}

impl ToTokens for RunOnBuildAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
    }
}
