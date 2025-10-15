use crate::__private::attribute::RewriteAttribute;
use crate::__private::auto_plugin_registry::_plugin_entry_block;
use crate::codegen::with_target_path::WithTargetPath;
use crate::macro_api::attributes::ItemAttributeArgs;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::with_plugin::{PluginBound, WithPlugin};
use crate::syntax::diagnostic::kind::item_kind;
use crate::util::macros::ok_or_return_compiler_error;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::{format_ident, quote};
use syn::{Item, parse2};

pub mod auto_bind_plugin;
pub mod auto_plugin;

fn body<T: PluginBound>(
    body: impl Fn(MacroStream) -> MacroStream,
) -> impl Fn(&Ident, T, &Item) -> syn::Result<MacroStream> {
    move |ident, params, _item| {
        let unique_ident = params.get_unique_ident(ident);
        let plugin = params.plugin().clone();
        let with_target_path = WithTargetPath::from((ident.into(), params));
        let output = with_target_path
            .to_tokens_iter()
            .enumerate()
            .map(|(ix, input)| {
                let body = body(input);
                let expr: syn::ExprClosure = syn::parse_quote!(|app| { #body });
                // required for generics
                let unique_ident = format_ident!("{unique_ident}_{ix}");
                let output = _plugin_entry_block(&unique_ident, &plugin, &expr);
                Ok(output)
            })
            .collect::<syn::Result<MacroStream>>()?;
        assert!(
            !output.is_empty(),
            "No plugin entry points were generated for ident: {ident}"
        );
        Ok(output)
    }
}

fn proc_attribute_inner<A, F>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
    parse_attr: fn(MacroStream) -> syn::Result<A>,
    body: F,
) -> MacroStream
where
    A: PluginBound,
    F: FnOnce(&Ident, A, &Item) -> syn::Result<MacroStream>,
{
    let attr = attr.into();
    let input = input.into();

    // need to clone input so we can pass through input untouched for optimal IDE support
    let item: Item = ok_or_return_compiler_error!(parse2(input.clone()));

    let err_msg = format!("Attribute macro is not allowed on {}", item_kind(&item));
    let ident = ok_or_return_compiler_error!(
        resolve_ident(&item).map_err(|e| {
            // make sure the call_site span is used instead so the user knows what attribute caused the error
            syn::Error::new(Span::call_site(), e)
        }),
        err_msg
    );

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote! {
        #input
        #output
    }
}

/// Maps [`crate::syntax::analysis::item::IdentFromItemResult`] to [`syn::Result<&Ident>`]
fn resolve_item_ident<T: PluginBound>(item: &Item) -> syn::Result<&Ident> {
    T::Inner::resolve_item_ident(item).map_err(syn::Error::from)
}

pub fn proc_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
) -> MacroStream
where
    T: PluginBound,
{
    proc_attribute_inner(
        attr,
        input,
        resolve_item_ident::<T>,
        parse2::<T>,
        body(|input| quote! { app #input ; }),
    )
}

pub fn proc_attribute_outer_call_fn<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
) -> MacroStream
where
    T: PluginBound,
{
    proc_attribute_inner(
        attr,
        input,
        resolve_item_ident::<T>,
        parse2::<T>,
        body(|input| quote! { #input(app) ; }),
    )
}

fn proc_attribute_rewrite_inner<T: RewriteAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> syn::Result<MacroStream> {
    use crate::macro_api::with_plugin::WithPlugin;
    let args = parse2::<WithPlugin<T>>(attr)?;
    let args_ts = args.inner.expand_attrs(&args.plugin());
    Ok(quote! {
        #args_ts
        #input
    })
}

pub fn proc_attribute_rewrite_outer<T: RewriteAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> MacroStream {
    proc_attribute_rewrite_inner::<T>(attr, input).unwrap_or_else(|err| err.to_compile_error())
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

macro_rules! gen_auto_attribute_outer_call_fns {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                proc_attribute_outer_call_fn::<WithPlugin<$args>>(attr, input)
            }
        )+
    };
}

macro_rules! gen_auto_attribute_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                proc_attribute_outer::<WithPlugin<$args>>(attr, input)
            }
        )+
    };
}

macro_rules! gen_auto_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                proc_attribute_rewrite_outer::<$args>(attr, input)
            }
        )+
    };
}

gen_auto_attribute_outer_call_fns! {
    auto_run_on_build         => RunOnBuildArgs,
}

gen_auto_attribute_outers! {
    auto_register_type        => RegisterTypeArgs,
    auto_add_message          => AddMessageArgs,
    auto_init_resource        => InitResourceArgs,
    auto_insert_resource      => InsertResourceArgs,
    auto_init_state           => InitStateArgs,
    auto_name                 => NameArgs,
    auto_register_state_type  => RegisterStateTypeArgs,
    auto_add_system           => AddSystemArgs,
    auto_add_observer         => AddObserverArgs,
    auto_add_plugin           => AddPluginArgs,
}

gen_auto_outers! {
    auto_component => ComponentArgs,
    auto_resource  => ResourceArgs,
    auto_system    => SystemArgs,
    auto_event     => EventArgs,
    auto_message   => MessageArgs,
    auto_observer  => ObserverArgs,
    auto_states    => StatesArgs,
}
