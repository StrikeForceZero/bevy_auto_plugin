use crate::codegen::ExpandAttrs;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::attributes::{
    AllowFn, AllowStructOrEnum, ItemAttribute, ItemAttributeContext,
};
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
    pub(crate) args: &'a mut T,
}

impl<'a, T> QQ<'a, T>
where
    T: ItemAttributeInput,
{
    pub(crate) fn from_args(args: &'a mut T) -> QQ<'a, T> {
        QQ::<'a, T> { args }
    }
}

pub trait QQToExpandAttr {
    fn to_expand_attr(&self, expand_attrs: &mut ExpandAttrs);
}

impl<'a, T> ToTokens for QQ<'a, T>
where
    Self: QQToExpandAttr,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        QQToExpandAttr::to_expand_attr(self, &mut expand_attr);
        expand_attr.to_tokens(tokens);
    }
}

impl<T> QQ<'_, T>
where
    T: MacroPathProvider + ItemAttributeInput + ItemAttributeContext,
    Self: ToTokens,
{
    pub fn inject_attribute_macro(&mut self) -> syn::Result<()> {
        let args = self.to_token_stream();
        let macro_path = T::macro_path(self.args.context()).clone();
        self.args.input_item_mut().map_ast(|item| {
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
