use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};
use syn::Path;

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct AutoPluginBuildHookArgs {
    hook: Path,
}

impl AttributeIdent for AutoPluginBuildHookArgs {
    const IDENT: &'static str = "auto_plugin_hook";
}

pub type IaAutoPluginBuildHook = ItemAttribute<
    Composed<AutoPluginBuildHookArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type AutoPluginBuildHookAppMutEmitter = AppMutationEmitter<IaAutoPluginBuildHook>;
pub type AutoPluginBuildHookAttrEmitter = AttrEmitter<IaAutoPluginBuildHook>;

impl EmitAppMutationTokens for AutoPluginBuildHookAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let custom = &self.args.args.base.hook;
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                <#custom as ::bevy_auto_plugin::__private::shared::AutoPluginBuildHook>::on_build::<#concrete_path>(#app_param);
            });
        }
        Ok(())
    }
}

impl ToTokens for AutoPluginBuildHookAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
