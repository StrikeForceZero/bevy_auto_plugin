use crate::codegen::ExpandAttrs;
use crate::macro_api::prelude::*;
use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote;
use syn::spanned::Spanned;

/// for codegen re-emitting macro args
#[derive(Debug, Clone)]
pub(crate) struct QQ<T> {
    pub(crate) args: T,
}

impl<T> QQ<T> {
    pub(crate) fn from_args(args: T) -> QQ<T> {
        QQ::<T> { args }
    }
}

pub trait QQToExpandAttr {
    fn to_expand_attr(&self, expand_attrs: &mut ExpandAttrs);
}

impl<T> ToTokens for QQ<T>
where
    Self: QQToExpandAttr,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        QQToExpandAttr::to_expand_attr(self, &mut expand_attr);
        expand_attr.to_tokens(tokens);
    }
}

impl<T> QQ<T>
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
