use bevy_auto_plugin_shared::module;
use proc_macro::TokenStream as CompilerStream;
use syn::{ItemMod, parse_macro_input};

/* Module */

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let mut attrs = bevy_auto_plugin_shared::module::attribute::AutoPluginAttributes::default();
    let arg_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(attr with arg_parser);

    // Parse the input module
    let module = parse_macro_input!(input as ItemMod);

    let injected_module = match module::inner::auto_plugin_inner(module, &attrs.init_name()) {
        Ok(code) => code,
        Err(err) => return err.to_compile_error().into(),
    };

    CompilerStream::from(injected_module)
}

/// Automatically registers a type with the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_register_type(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds an event type to the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_add_event(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a resource in the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_init_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_name(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a State in the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_init_state(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically registers a State<T> and NextState<T> in the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_register_state_type(
    _attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/* Flat File */

use bevy_auto_plugin_shared::flat_file;
use bevy_auto_plugin_shared::flat_file::inner::auto_plugin_inner;
use bevy_auto_plugin_shared::util::{Target, resolve_local_file};
use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Error, Item, ItemFn, Path, Token};

/// Attaches to a function accepting `&mut bevy::prelude::App`, automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let mut attrs = flat_file::attribute::AutoPluginAttributes::default();
    let arg_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(attr with arg_parser);
    let Some(app_param_name) = attrs.app_param_name else {
        return Error::new(
            attrs.app_param_name.span(),
            "auto_plugin requires attribute specifying the name of the `&mut bevy::app::App` parameter. Example: #[auto_plugin(app=app)]",
        )
            .into_compile_error()
            .into();
    };

    // Parse the input function
    let input = parse_macro_input!(input as ItemFn);

    let path = match resolve_local_file(
        #[cfg(feature = "lang_server_noop")]
        input.to_token_stream()
    ) {
        Ok(path) => path,
        Err(ts) => return ts.into(),
    };

    CompilerStream::from(
        auto_plugin_inner(path, input, app_param_name).unwrap_or_else(|err| err.to_compile_error()),
    )
}

fn flat_file_handle_attribute(
    attr: CompilerStream,
    input: CompilerStream,
    target: Target,
) -> CompilerStream {
    let cloned_input = input.clone();
    let parsed_item = parse_macro_input!(input as Item);
    let args = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr with Punctuated::<Path, Token![,]>::parse_terminated))
    };

    let path = match resolve_local_file(
        #[cfg(feature = "lang_server_noop")]
        parsed_item.to_token_stream()
    ) {
        Ok(path) => path,
        Err(ts) => return ts.into(),
    };

    flat_file::inner::handle_attribute_inner(path, parsed_item, Span::call_site(), target, args)
        .map(|_| cloned_input)
        .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Automatically registers a type with the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::RegisterTypes)
}
/// Automatically adds an event type to the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::AddEvents)
}
/// Automatically initializes a resource in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::InitResources)
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::RequiredComponentAutoName)
}

/// Automatically initializes a State in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::InitStates)
}

/// Automatically registers a State type in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    flat_file_handle_attribute(attr, input, Target::RegisterStateTypes)
}
