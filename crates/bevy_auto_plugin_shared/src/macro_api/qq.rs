use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{AllowFn, AllowStructOrEnum, ItemAttribute};
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::input_item::InputItem;
use crate::macro_api::macro_paths::MacroPathProvider;
use crate::macro_api::mixins::generics::none::WithNoGenerics;
use crate::macro_api::mixins::generics::with_many::WithZeroOrManyGenerics;
use crate::macro_api::mixins::generics::with_single::WithZeroOrOneGenerics;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

/// for codegen re-emitting macro args
pub(crate) struct QQ<'a, T> {
    pub(crate) args: &'a T,
    pub(crate) context: &'a Context,
    pub(crate) input_item: &'a mut InputItem,
}

impl<T> QQ<'_, T>
where
    T: MacroPathProvider,
    Self: ToTokens,
{
    pub fn inject_attribute_macro(&mut self) -> syn::Result<()> {
        let args = self.to_token_stream();
        self.input_item.map_ast(|item| {
            let macro_path = T::macro_path(self.context);
            // insert attribute tokens
            let mut attrs = item
                .take_attrs()
                .map_err(|err| syn::Error::new(item.span(), err))?;
            attrs.insert(0, parse_quote!(#[#macro_path(#args)]));
            item.put_attrs(attrs).unwrap(); // infallible
            Ok(())
        })
    }
}
