use crate::__private::expand::attr;
use crate::macro_api::prelude::*;
use crate::syntax::extensions::item::ItemAttrsExt;
use crate::util::macros::compile_error_with;
use proc_macro2::TokenStream as MacroStream;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn auto_bind_plugin_inner(
    attr: MacroStream,
    input: MacroStream,
    context: Context,
) -> syn::Result<MacroStream> {
    // TODO: need to determine correct flow to maintain input tokens for errors
    let mut item_attribute =
        ItemAttribute::<Composed<Nothing, WithPlugin>, AllowAny>::from_attr_input(
            attr,
            input.clone(),
            context,
        )?;

    let plugin_path = item_attribute.args.plugin();
    let item = item_attribute.input_item.ensure_ast_mut()?;
    let mut attrs = item
        .take_attrs()
        .map_err(|err| syn::Error::new(item.span(), err))?;

    attr::attrs_inject_plugin_param(&mut attrs, plugin_path);

    let Ok(_) = item.put_attrs(attrs) else {
        unreachable!()
    };

    Ok(item.to_token_stream())
}

pub fn auto_bind_plugin_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    let og_input = input.clone();
    auto_bind_plugin_inner(attr, input, Context::default())
        .unwrap_or_else(|err| compile_error_with!(err, og_input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use quote::quote;
    #[xtest]
    fn test_auto_bind_plugin_inner() {
        let attr = quote!(plugin = Test);
        let input = quote! {
            #[derive(Component, Reflect)]
            #[reflect(Component)]
            #[auto_register_type]
            #[some::path::auto_name]
            struct FooComponent;
        };
        assert_eq!(
            auto_bind_plugin_outer(attr, input).to_string(),
            quote! {
                # [derive (Component , Reflect)]
                # [reflect (Component)]
                # [auto_register_type (plugin = Test)]
                # [some::path::auto_name (plugin = Test)]
                struct FooComponent ;
            }
            .to_string()
        );
    }
}
