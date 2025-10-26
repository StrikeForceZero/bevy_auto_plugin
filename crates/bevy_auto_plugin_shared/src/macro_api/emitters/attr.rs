use crate::codegen::ExpandAttrs;
use crate::macro_api::prelude::*;
use crate::syntax::extensions::item::ItemAttrsExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse_quote;
use syn::spanned::Spanned;

/// for codegen re-emitting macro args
#[derive(Debug, Clone)]
pub(crate) struct AttrEmitter<T> {
    pub(crate) args: T,
}

impl<T> AttrEmitter<T> {
    pub(crate) fn from_args(args: T) -> AttrEmitter<T> {
        AttrEmitter::<T> { args }
    }
}

impl<T, P, G, R> AttrEmitter<ItemAttribute<Composed<T, P, G>, R>>
where
    T: MacroPathProvider,
{
    pub(crate) fn wrap_as_attr(&self, args: &TokenStream) -> TokenStream {
        let macro_path = T::macro_path(self.args.context());
        if args.is_empty() {
            quote! { #[#macro_path] }
        } else {
            quote! { #[#macro_path( #args )] }
        }
    }
}

pub trait AttrEmitterToExpandAttr {
    fn to_expand_attr(&self, expand_attrs: &mut ExpandAttrs);
}

impl<T> ToTokens for AttrEmitter<T>
where
    Self: AttrEmitterToExpandAttr,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        AttrEmitterToExpandAttr::to_expand_attr(self, &mut expand_attr);
        expand_attr.to_tokens(tokens);
    }
}

impl<T> AttrEmitter<T>
where
    T: MacroPathProvider + ItemAttributeInput + ItemAttributeContext,
    Self: ToTokens,
{
    // TODO: replace the one ins expand/attr
    #[allow(dead_code)]
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
