use crate::__private::attribute::ShortHandAttribute;
use crate::__private::auto_plugin_registry::_plugin_entry_block;
use crate::codegen::with_target_path::WithTargetPath;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::global_args::{GlobalArgs, GlobalAttributeArgs, ItemAttributeArgs};
use crate::ok_or_return_compiler_error;
use crate::syntax::diagnostic::kind::item_kind;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::quote;
use syn::{Item, parse2};

pub mod auto_bind_plugin;
pub mod auto_plugin;

fn proc_attribute_inner<A, F>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
    parse_attr: fn(MacroStream) -> syn::Result<A>,
    body: F,
) -> MacroStream
where
    A: GlobalAttributeArgs,
    F: FnOnce(&Ident, A, &Item) -> syn::Result<MacroStream>,
{
    let attr = attr.into();
    let input = input.into();

    let item: Item = ok_or_return_compiler_error!(parse2(input));

    let err_msg = format!("Attribute macro is not allowed on {}", item_kind(&item));
    let ident = ok_or_return_compiler_error!(resolve_ident(&item), err_msg);

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote!( #item #output )
}

pub fn proc_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
) -> MacroStream
where
    T: GlobalAttributeArgs,
{
    /// Maps [`crate::syntax::analysis::item::IdentFromItemResult`] to [`syn::Result<&Ident>`]
    fn resolve_item_ident<T: GlobalAttributeArgs>(item: &Item) -> syn::Result<&Ident> {
        T::Inner::resolve_item_ident(item).map_err(|err| syn::Error::new(Span::call_site(), err))
    }

    proc_attribute_inner(
        attr,
        input,
        resolve_item_ident::<T>,
        parse2::<T>,
        |ident, params, _item| {
            let unique_ident = params.get_unique_ident(ident);
            let plugin = params.plugin().clone();
            let with_target_path = WithTargetPath::from((ident.into(), params));
            let output = with_target_path
                .to_tokens_iter()
                .map(|input| {
                    let register = quote! { app #input ; };
                    let expr: syn::ExprClosure = syn::parse_quote!(|app| { #register });
                    let output = _plugin_entry_block(&unique_ident, &plugin, &expr);
                    Ok(output)
                })
                .collect::<syn::Result<MacroStream>>()?;
            assert!(
                !output.is_empty(),
                "No plugin entry points were generated for ident: {ident}"
            );
            Ok(output)
        },
    )
}

fn auto_inner<T: ShortHandAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> syn::Result<MacroStream> {
    use crate::macro_api::global_args::GlobalArgs;
    let args = parse2::<GlobalArgs<T>>(attr)?;
    let args_ts = args.inner.expand_attrs(&args.plugin());
    Ok(quote! {
        #args_ts
        #input
    })
}

pub fn auto_outer<T: ShortHandAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> MacroStream {
    auto_inner::<T>(attr, input).unwrap_or_else(|err| err.to_compile_error())
}

pub fn inject_plugin_arg_for_attributes(attrs: &mut Vec<syn::Attribute>, plugin: &syn::Path) {
    use syn::Meta;

    for attr in attrs {
        let last = attr
            .path()
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();

        if !last.starts_with("auto_") {
            continue;
        }

        let already_has_plugin = match &attr.meta {
            Meta::List(ml) => list_has_key(ml, "plugin"),
            Meta::Path(_) => false,
            Meta::NameValue(_) => true,
        };

        if already_has_plugin {
            continue;
        }

        inject_plugin_arg(attr, plugin);
    }
}

fn inject_plugin_arg(attr: &mut syn::Attribute, plugin: &syn::Path) {
    use syn::Meta;
    use syn::parse_quote;
    match &attr.meta {
        Meta::Path(path) => *attr = parse_quote!( #[#path(plugin = #plugin)] ),
        Meta::List(ml) => {
            let path = &ml.path;
            let inner = &ml.tokens;
            if inner.is_empty() {
                *attr = parse_quote!( #[#path(plugin = #plugin)] )
            } else {
                *attr = parse_quote!( #[#path(plugin = #plugin, #inner)] )
            }
        }
        _ => {}
    }
}

fn list_has_key(ml: &syn::MetaList, key: &str) -> bool {
    use syn::Meta;
    use syn::Token;
    use syn::parse::Parser;
    use syn::punctuated::Punctuated;
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

pub fn auto_register_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<RegisterTypeArgs>>(attr, input)
}

pub fn auto_add_message_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<AddMessageArgs>>(attr, input)
}

pub fn auto_init_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<InitResourceArgs>>(attr, input)
}

pub fn auto_insert_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<InsertResourceArgs>>(attr, input)
}

pub fn auto_init_state_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<InitStateArgs>>(attr, input)
}

pub fn auto_name_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<NameArgs>>(attr, input)
}

pub fn auto_register_state_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<RegisterStateTypeArgs>>(attr, input)
}

pub fn auto_add_system_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<AddSystemArgs>>(attr, input)
}

pub fn auto_add_observer_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    proc_attribute_outer::<GlobalArgs<AddObserverArgs>>(attr, input)
}

macro_rules! gen_auto_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                auto_outer::<$args>(attr, input)
            }
        )+
    };
}

gen_auto_outers! {
    auto_component => ComponentAttributeArgs,
    auto_resource  => ResourceAttributeArgs,
    auto_system    => SystemAttributeArgs,
    auto_event     => EventArgs,
    auto_message   => MessageAttributeArgs,
    auto_observer  => ObserverAttributeArgs,
    auto_states    => StatesAttributeArgs,
}
