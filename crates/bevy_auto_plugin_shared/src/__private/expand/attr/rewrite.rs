use crate::macro_api::prelude::*;
use crate::util::macros::ok_or_emit_with;
use proc_macro2::TokenStream as MacroStream;
use quote::ToTokens;

pub fn proc_attribute_rewrite_outer<T>(attr: MacroStream, input: MacroStream) -> MacroStream
where
    AttrExpansionEmitter<T>: ToTokens,
    T: ItemAttributeArgs + ItemAttributeParse + ItemAttributeInput + ItemAttributeContext,
{
    let args = ok_or_emit_with!(
        T::from_attr_input_with_context(attr, input.clone(), Context::default()),
        input
    );
    AttrExpansionEmitter::from_item_attribute(args).to_token_stream()
}
