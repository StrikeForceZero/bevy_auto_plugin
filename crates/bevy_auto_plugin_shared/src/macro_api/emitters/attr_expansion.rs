use crate::codegen::ExpandAttrs;
use crate::macro_api::prelude::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::marker::PhantomData;

/// for codegen rewriting attrs
#[derive(Debug, Clone)]
pub struct AttrExpansionEmitter<T> {
    pub(crate) args: T,
}

impl<T> AttrExpansionEmitter<T>
where
    T: ItemAttributeParse,
{
    pub fn from_item_attribute(item_attribute: T) -> AttrExpansionEmitter<T> {
        AttrExpansionEmitter::<T> {
            args: item_attribute,
        }
    }
}

pub trait AttrExpansionEmitterToExpandAttr {
    fn to_expand_attrs(&self, expand_attrs: &mut ExpandAttrs);
}

impl<T> ToTokens for AttrExpansionEmitter<T>
where
    Self: AttrExpansionEmitterToExpandAttr,
    T: ItemAttributeInput,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut expand_attr = ExpandAttrs::default();
        AttrExpansionEmitterToExpandAttr::to_expand_attrs(self, &mut expand_attr);
        tokens.extend(expand_attr.to_token_stream());
        tokens.extend(self.args.input_item().to_token_stream());
    }
}

impl<TFrom, TTo, P, GFrom, GTo, RFrom, RTo>
    From<AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>>
    for AttrEmitter<ItemAttribute<Composed<TTo, P, GTo>, RTo>>
where
    TTo: From<TFrom>,
    GTo: From<GFrom>,
    RTo: From<RFrom>,
{
    fn from(value: AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>) -> Self {
        let ItemAttribute {
            args,
            context,
            input_item,
            target,
            _resolver,
        } = value.args;

        let mapped = Composed {
            base: args.base.into(),         // TTo: From<TFrom>
            plugin: args.plugin,            // same P
            generics: args.generics.into(), // GTo: From<GFrom>
        };

        let args = ItemAttribute::<Composed<TTo, P, GTo>, RTo> {
            args: mapped,
            context, // RTo: From<RFrom>
            input_item,
            target,
            _resolver: PhantomData,
        };

        Self::from_args(args)
    }
}

impl<TFrom, TTo, P, GFrom, GTo, RFrom, RTo>
    From<&AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>>
    for AttrEmitter<ItemAttribute<Composed<TTo, P, GTo>, RTo>>
where
    TTo: From<TFrom>,
    GTo: From<GFrom>,
    RTo: From<RFrom>,
    AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>: Clone,
{
    fn from(value: &AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>) -> Self {
        <Self as From<AttrExpansionEmitter<ItemAttribute<Composed<TFrom, P, GFrom>, RFrom>>>>::from(
            value.clone(),
        )
    }
}
