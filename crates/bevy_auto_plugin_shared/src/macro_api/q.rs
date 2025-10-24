use crate::macro_api::attributes::ItemAttributeParse;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};

/// for codegen attaching to bevy app
pub(crate) struct Q<'a, T> {
    pub(crate) args: &'a T,
    // TODO: maybe app params should just be part of another wrapper struct?
    pub(crate) app_param: syn::Ident,
}

impl<T> Q<'_, T>
where
    T: ItemAttributeParse,
{
    pub fn from_args(args: &T) -> Q<T> {
        Q::<T> {
            args,
            app_param: format_ident!("app"),
        }
    }
}

pub trait ToTokensWithAppParam {
    fn to_tokens(&self, tokens: &mut TokenStream, app_param: &syn::Ident);
}

impl<'a, T> ToTokens for Q<'a, T>
where
    Self: ToTokensWithAppParam,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        ToTokensWithAppParam::to_tokens(self, tokens, &self.app_param);
    }
}
