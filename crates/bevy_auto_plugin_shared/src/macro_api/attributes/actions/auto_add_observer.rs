use crate::macro_api::prelude::*;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    quote,
};

#[derive(FromMeta, Debug, Default, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct AddObserverArgs {}

impl AttributeIdent for AddObserverArgs {
    const IDENT: &'static str = "auto_add_observer";
}

pub type IaAddObserver =
    ItemAttribute<Composed<AddObserverArgs, WithPlugin, WithZeroOrManyGenerics>, AllowFn>;
pub type AddObserverAppMutEmitter = AppMutationEmitter<IaAddObserver>;
pub type AddObserverAttrEmitter = AttrEmitter<IaAddObserver>;

impl EmitAppMutationTokens for AddObserverAppMutEmitter {
    fn to_app_mutation_tokens(
        &self,
        tokens: &mut TokenStream,
        app_param: &syn::Ident,
    ) -> syn::Result<()> {
        let concrete_paths = self.args.concrete_paths()?;
        for concrete_path in concrete_paths {
            tokens.extend(quote! {
                #app_param.add_observer( #concrete_path );
            });
        }
        Ok(())
    }
}

impl ToTokens for AddObserverAttrEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = self.args.args.extra_args();
        tokens.extend(quote! {
            #(#args),*
        });
        *tokens = self.wrap_as_attr(tokens);
    }
}
