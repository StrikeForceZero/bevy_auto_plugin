use crate::{
    macro_api::prelude::*,
    util::macros::ok_or_emit_with,
};
use proc_macro2::TokenStream as MacroStream;
use quote::{
    ToTokens,
    quote,
};

pub fn proc_attribute_outer<T>(attr: MacroStream, input: MacroStream) -> MacroStream
where
    T: ItemAttributeArgs
        + ItemAttributeParse
        + ItemAttributeInput
        + ItemAttributeTarget
        + ItemAttributeUniqueIdent
        + ItemAttributeContext
        + ItemAttributePlugin,
    AppMutationEmitter<T>: ToTokens + EmitAppMutationTokens,
{
    let args = ok_or_emit_with!(
        T::from_attr_input_with_context(attr, input.clone(), Context::default()),
        input
    );
    let mut app_mut_emitter = AppMutationEmitter::from_args(args);
    let processed_item = {
        ok_or_emit_with!(app_mut_emitter.item_post_process(), app_mut_emitter.args.input_item());
        app_mut_emitter.args.input_item().to_token_stream()
    };
    let after_item_tokens =
        ok_or_emit_with!(app_mut_emitter.wrap_body(|body| quote! { #body }), processed_item);
    quote! {
        #processed_item
        #after_item_tokens
    }
}
