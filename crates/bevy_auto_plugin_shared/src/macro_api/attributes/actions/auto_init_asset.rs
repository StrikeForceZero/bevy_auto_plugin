use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct InitAssetArgs {}

impl AttributeIdent for InitAssetArgs {
    const IDENT: &'static str = "auto_init_asset";
}

pub type IaInitAsset = ItemAttribute<
    Composed<InitAssetArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnumOrUse,
>;
pub type InitAssetAppMutEmitter = AppMutationEmitter<IaInitAsset>;
pub type InitAssetAttrEmitter = AttrEmitter<IaInitAsset>;

impl EmitAppMutationTokens for InitAssetAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let bevy_asset = crate::__private::paths::asset::asset_root_path();
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #bevy_asset::AssetApp::init_asset::<#concrete_path>(#app_param);
            });
        }
        Ok(())
    }
}

impl ToTokens for InitAssetAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
