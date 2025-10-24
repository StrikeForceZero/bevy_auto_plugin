use crate::codegen::ExpandAttrs;
use crate::macro_api::prelude::*;
use proc_macro2::TokenStream;
use quote::ToTokens;

/// for codegen rewriting attrs
#[derive(Debug, Clone)]
pub(crate) struct RewriteQ<T> {
    pub(crate) args: T,
}

impl<T> RewriteQ<T>
where
    T: ItemAttributeParse,
{
    pub fn from_item_attribute(item_attribute: T) -> RewriteQ<T> {
        RewriteQ::<T> {
            args: item_attribute,
        }
    }
}

pub trait RewriteQToExpandAttr {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs);
}

impl<T> ToTokens for RewriteQ<T>
where
    Self: RewriteQToExpandAttr,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        RewriteQToExpandAttr::to_expand_attrs(self, &mut expand_attr);
        expand_attr.to_tokens(tokens);
    }
}

impl<TFrom, TTo, P, G, R> From<RewriteQ<ItemAttribute<Composed<TFrom, P, G>, R>>>
    for QQ<ItemAttribute<Composed<TTo, P, G>, R>>
where
    TTo: From<TFrom>,
{
    fn from(value: RewriteQ<ItemAttribute<Composed<TFrom, P, G>, R>>) -> Self {
        let args = ItemAttribute::<ConvertComposed<Composed<TTo, P, G>>, R>::from(value.args);
        let args = ItemAttribute::<Composed<TTo, P, G>, R>::from(args);
        Self::from_args(args)
    }
}

impl<TFrom, TTo, P, G, R> From<&RewriteQ<ItemAttribute<Composed<TFrom, P, G>, R>>>
    for QQ<ItemAttribute<Composed<TTo, P, G>, R>>
where
    TTo: From<TFrom>,
    RewriteQ<ItemAttribute<Composed<TFrom, P, G>, R>>: Clone,
{
    fn from(value: &RewriteQ<ItemAttribute<Composed<TFrom, P, G>, R>>) -> Self {
        Self::from(value.clone())
    }
}
