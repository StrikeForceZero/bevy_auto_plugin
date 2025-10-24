use crate::codegen::ExpandAttrs;
use crate::macro_api::prelude::*;
use proc_macro2::TokenStream;
use quote::ToTokens;

/// for codegen rewriting attrs
pub(crate) struct RewriteQ<'a, T> {
    pub(crate) args: &'a mut T,
}

impl<T> RewriteQ<'_, T>
where
    T: ItemAttributeParse,
{
    pub fn from_item_attribute(item_attribute: &mut T) -> RewriteQ<T> {
        RewriteQ::<T> {
            args: item_attribute,
        }
    }
}

pub trait RewriteQToExpandAttr {
    fn to_expand_attr(&self, expand_attrs: &mut ExpandAttrs);
}

impl<'a, T> ToTokens for RewriteQ<'a, T>
where
    Self: RewriteQToExpandAttr,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        RewriteQToExpandAttr::to_expand_attr(self, &mut expand_attr);
        expand_attr.to_tokens(tokens);
    }
}

impl<'a, TFrom, TTo, P, G, R> From<RewriteQ<'a, ItemAttribute<Composed<TFrom, P, G>, R>>>
    for QQ<'a, ItemAttribute<Composed<TTo, P, G>, R>>
where
    TTo: From<TFrom>,
{
    fn from(value: RewriteQ<'a, ItemAttribute<Composed<TFrom, P, G>, R>>) -> Self {
        QQ::<'a, ItemAttribute<Composed<TTo, P, G>, R>>::from_args(TTo::from(value.args))
    }
}
