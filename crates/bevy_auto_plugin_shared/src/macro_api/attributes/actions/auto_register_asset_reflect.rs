use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterAssetReflectArgs {}

impl AttributeIdent for RegisterAssetReflectArgs {
    const IDENT: &'static str = "auto_register_asset_reflect";
}

pub type IaRegisterAssetReflect = ItemAttribute<
    Composed<RegisterAssetReflectArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnumOrUse,
>;
pub type RegisterAssetReflectAppMutEmitter = AppMutationEmitter<IaRegisterAssetReflect>;
pub type RegisterAssetReflectAttrEmitter = AttrEmitter<IaRegisterAssetReflect>;

impl EmitAppMutationTokens for RegisterAssetReflectAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let bevy_asset = crate::__private::paths::asset::asset_root_path();
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #bevy_asset::AssetApp::register_asset_reflect::<#concrete_path>(#app_param);
            });
        }
        Ok(())
    }
}

impl ToTokens for RegisterAssetReflectAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
