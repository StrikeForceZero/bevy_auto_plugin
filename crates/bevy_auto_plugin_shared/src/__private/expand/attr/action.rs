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
        + Clone
        + ItemAttributeParse
        + ItemAttributeInput
        + ItemAttributeTarget
        + ItemAttributeTargetMut
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
        if let Err((tokens, err)) = app_mut_emitter.post_process_inner_item() {
            let err = err.to_compile_error();
            return quote! {
                #err
                #tokens
            };
        }
        app_mut_emitter.args.input_item().to_token_stream()
    };
    let use_targets = ok_or_emit_with!(
        app_mut_emitter.args.input_item().use_target_paths(),
        processed_item.clone()
    );
    let after_item_tokens = if let Some(use_targets) = use_targets {
        let mut tokens = MacroStream::new();
        for target in use_targets {
            let mut args = app_mut_emitter.args.clone();
            args.set_target(target);
            let mut per_target_emitter =
                AppMutationEmitter { args, app_param: app_mut_emitter.app_param.clone() };
            let entry_tokens = ok_or_emit_with!(
                per_target_emitter.wrap_body(|body| quote! { #body }),
                processed_item.clone()
            );
            tokens.extend(entry_tokens);
        }
        tokens
    } else {
        ok_or_emit_with!(app_mut_emitter.wrap_body(|body| quote! { #body }), processed_item)
    };
    quote! {
        #processed_item
        #after_item_tokens
    }
}
