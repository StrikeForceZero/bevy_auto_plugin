use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct RegisterTypeArgs {}

impl AttributeIdent for RegisterTypeArgs {
    const IDENT: &'static str = "auto_register_type";
}

pub type IaRegisterType = ItemAttribute<
    Composed<RegisterTypeArgs, WithPlugin, WithZeroOrManyGenerics>,
    AllowStructOrEnum,
>;
pub type RegisterTypeAppMutEmitter = AppMutationEmitter<IaRegisterType>;
pub type RegisterTypeAttrEmitter = AttrEmitter<IaRegisterType>;

impl EmitAppMutationTokens for RegisterTypeAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #app_param.register_type::<#concrete_path>();
            });
        }
        Ok(())
    }
}
impl ToTokens for RegisterTypeAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
