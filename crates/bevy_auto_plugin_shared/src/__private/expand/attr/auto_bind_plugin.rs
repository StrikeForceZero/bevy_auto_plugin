use crate::__private::expand::attr;
use crate::macro_api::attributes::ItemAttribute;
use crate::macro_api::composed::Composed;
use crate::macro_api::context::Context;
use crate::macro_api::mixins::nothing::Nothing;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::extensions::item::ItemAttrsExt;
use crate::util::macros::{compile_error_with, ok_or_emit_with};
use proc_macro2::TokenStream as MacroStream;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn auto_bind_plugin_inner(
    attr: MacroStream,
    input: MacroStream,
    context: Context,
) -> syn::Result<MacroStream> {
    // TODO: need to determine correct flow to maintain input tokens for errors
    let mut item_attribute = ItemAttribute::<Composed<Nothing, WithPlugin>, _>::from_attr_input(
        attr,
        input.clone(),
        context,
    )
    .map_err(|err| compile_error_with!(err, input.clone()))?;

    let plugin_path = item_attribute.args.plugin();
    let mut item = item_attribute.input_item.ensure_ast_mut()?;
    let mut attrs = item
        .take_attrs()
        .map_err(|err| syn::Error::new(item.span(), err))?;

    attr::inject_plugin_arg_for_attributes(&mut attrs, plugin_path);

    let Ok(_) = item.put_attrs(attrs) else {
        unreachable!()
    };

    Ok(item_attribute.input_item.to_token_stream())
}

pub fn auto_bind_plugin_outer(
    attr: MacroStream,
    input: MacroStream,
    context: Context,
) -> MacroStream {
    let og_input = input.clone();
    auto_bind_plugin_inner(attr, input, context)
        .unwrap_or_else(|err| compile_error_with!(err, og_input))
}
