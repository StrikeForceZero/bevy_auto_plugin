use crate::{
    macro_api::prelude::*,
    syntax::extensions::item::ItemAttrsExt,
    util::macros::compile_error_with,
};
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
    let plugin_after_build = item_attribute.args.plugin.after_build.is_present();
    let item = item_attribute.input_item.ensure_ast_mut()?;
    let mut attrs = item.take_attrs().map_err(|err| syn::Error::new(item.span(), err))?;

    attrs_inject_plugin_param(&mut attrs, plugin_path, plugin_after_build);

    let Ok(_) = item.put_attrs(attrs) else { unreachable!() };

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

pub fn attrs_inject_plugin_param(
    attrs: &mut Vec<syn::Attribute>,
    plugin: &syn::Path,
    plugin_after_build: bool,
) {
    use syn::Meta;

    for attr in attrs {
        let last = attr.path().segments.last().map(|s| s.ident.to_string()).unwrap_or_default();

        if !last.starts_with("auto_") {
            continue;
        }

        let already_has_plugin = match &attr.meta {
            Meta::List(ml) => list_has_key(ml, "plugin"),
            Meta::Path(_) => false,
            Meta::NameValue(_) => true,
        };
        let already_has_after_build = match &attr.meta {
            Meta::List(ml) => list_has_key(ml, "after_build"),
            Meta::Path(_) => false,
            Meta::NameValue(_) => true,
        };

        let add_plugin = !already_has_plugin;
        let add_after_build = plugin_after_build && !already_has_after_build;

        if !add_plugin && !add_after_build {
            continue;
        }

        attr_inject_with_plugin_params(attr, plugin, add_plugin, add_after_build);
    }
}

fn attr_inject_with_plugin_params(
    attr: &mut syn::Attribute,
    plugin: &syn::Path,
    add_plugin: bool,
    add_after_build: bool,
) {
    use syn::{
        Meta,
        Token,
        parse::Parser,
        parse_quote,
        punctuated::Punctuated,
    };
    match &attr.meta {
        Meta::Path(path) => {
            *attr = if add_plugin && add_after_build {
                parse_quote!( #[#path(plugin = #plugin, after_build)] )
            } else if add_plugin {
                parse_quote!( #[#path(plugin = #plugin)] )
            } else if add_after_build {
                parse_quote!( #[#path(after_build)] )
            } else {
                return;
            };
        }
        Meta::List(ml) => {
            let path = &ml.path;
            let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
            match parser.parse2(ml.tokens.clone()) {
                Ok(list) => {
                    let mut items: Vec<Meta> = list.into_iter().collect();
                    if add_plugin {
                        items.insert(0, parse_quote!(plugin = #plugin));
                    }
                    if add_after_build {
                        items.push(parse_quote!(after_build));
                    }
                    let tokens = quote::quote! { #(#items),* };
                    *attr = parse_quote!( #[#path( #tokens )] );
                }
                Err(_) => {
                    let inner = &ml.tokens;
                    *attr = match (add_plugin, add_after_build) {
                        (true, true) => {
                            if inner.is_empty() {
                                parse_quote!( #[#path(plugin = #plugin, after_build)] )
                            } else {
                                parse_quote!( #[#path(plugin = #plugin, after_build, #inner)] )
                            }
                        }
                        (true, false) => {
                            if inner.is_empty() {
                                parse_quote!( #[#path(plugin = #plugin)] )
                            } else {
                                parse_quote!( #[#path(plugin = #plugin, #inner)] )
                            }
                        }
                        (false, true) => {
                            if inner.is_empty() {
                                parse_quote!( #[#path(after_build)] )
                            } else {
                                parse_quote!( #[#path(after_build, #inner)] )
                            }
                        }
                        (false, false) => return,
                    };
                }
            }
        }
        _ => {}
    }
}

fn list_has_key(ml: &syn::MetaList, key: &str) -> bool {
    use syn::{
        Meta,
        Token,
        parse::Parser,
        punctuated::Punctuated,
    };
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    match parser.parse2(ml.tokens.clone()) {
        Ok(list) => list.iter().any(|m| match m {
            Meta::NameValue(nv) => nv.path.is_ident(key),
            Meta::List(ml2) => ml2.path.is_ident(key),
            Meta::Path(p) => p.is_ident(key),
        }),
        Err(_) => false,
    }
}
