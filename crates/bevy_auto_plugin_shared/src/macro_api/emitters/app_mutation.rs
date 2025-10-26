use crate::macro_api::prelude::*;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};

/// for codegen attaching to bevy app
#[derive(Debug, Clone)]
pub(crate) struct AppMutationEmitter<T> {
    pub(crate) args: T,
    // TODO: maybe app params should just be part of another wrapper struct?
    pub(crate) app_param: syn::Ident,
}

impl<T> AppMutationEmitter<T> {
    pub fn from_args(args: T) -> AppMutationEmitter<T> {
        AppMutationEmitter::<T> {
            args,
            app_param: format_ident!("app"),
        }
    }
}

pub trait EmitAppMutationTokens {
    fn scrub_item(&mut self) -> syn::Result<()> {
        Ok(())
    }
    fn to_app_mutation_token_stream(&self, app_param: &syn::Ident) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_app_mutation_tokens(&mut tokens, app_param);
        tokens
    }
    fn to_app_mutation_tokens(&self, out: &mut TokenStream, app_param: &syn::Ident);
}

impl<T> ToTokens for AppMutationEmitter<T>
where
    Self: EmitAppMutationTokens,
    T: ItemAttributeInput,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        EmitAppMutationTokens::to_app_mutation_tokens(self, tokens, &self.app_param)
    }
}
