use crate::macro_api::prelude::*;
use crate::util::macros::ok_or_emit_with;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};

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
    let mut q = AppMutationEmitter::from_args(args);
    let scrubbed_input = {
        ok_or_emit_with!(q.scrub_item(), q.args.input_item());
        q.args.input_item().to_token_stream()
    };
    let after_item_tokens = ok_or_emit_with!(q.wrap_body(|body| quote! { #body }), scrubbed_input);
    quote! {
        #scrubbed_input
        #after_item_tokens
    }
}
