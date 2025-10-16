use crate::util::macros::compile_error_with;
use proc_macro2::TokenStream as MacroStream;

pub fn auto_bind_plugin_inner(attr: MacroStream, input: MacroStream) -> syn::Result<MacroStream> {
    use crate::__private::expand::attr;
    use crate::macro_api::with_plugin::WithPlugin;
    use crate::syntax::extensions::item::ItemAttrsExt;
    use proc_macro2::Span;
    use quote::quote;
    use syn::{Item, parse2};

    let mut item = parse2::<Item>(input)?;
    let args = parse2::<WithPlugin<()>>(attr)?;
    let plugin = args.plugin;

    let Ok(mut attrs) = item.take_attrs() else {
        return Err(syn::Error::new(
            Span::call_site(),
            "auto_bind_plugin supports only functions, structs, or enums",
        ));
    };

    attr::inject_plugin_arg_for_attributes(&mut attrs, &plugin);

    let Ok(_) = item.put_attrs(attrs) else {
        unreachable!()
    };

    Ok(quote! { #item })
}

pub fn auto_bind_plugin_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    let og_input = input.clone();
    auto_bind_plugin_inner(attr, input).unwrap_or_else(|err| compile_error_with!(err, og_input))
}
