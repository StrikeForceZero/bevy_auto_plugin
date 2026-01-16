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
pub struct AutoPluginCustomArgs {
    custom: Path,
}

impl AttributeIdent for AutoPluginCustomArgs {
    const IDENT: &'static str = "auto_plugin_custom";
}

pub type IaAutoPluginCustom = ItemAttribute<
    Composed<AutoPluginCustomArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type AutoPluginCustomAppMutEmitter = AppMutationEmitter<IaAutoPluginCustom>;
pub type AutoPluginCustomAttrEmitter = AttrEmitter<IaAutoPluginCustom>;

impl EmitAppMutationTokens for AutoPluginCustomAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let custom = &self.args.args.base.custom;
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                <#custom as ::bevy_auto_plugin::__private::shared::__private::auto_plugin_registry::AutoPluginCustom>::on_build::<#concrete_path>(#app_param);
            });
        }
        Ok(())
    }
}

impl ToTokens for AutoPluginCustomAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
