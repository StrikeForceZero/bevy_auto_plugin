use bevy_auto_plugin_shared::module::inner::expand_module;
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;
use syn::{Error, parse_macro_input};

fn to_compile_error(err: Error) -> MacroStream {
    err.to_compile_error()
}

/* Module */

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    expand_module(attr.into(), input.into())
        .unwrap_or_else(to_compile_error)
        .into()
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

/// Automatically add_system in the Bevy `App`.
#[proc_macro_attribute]
pub fn module_auto_add_system(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/* Flat File */

use bevy_auto_plugin_shared::flat_file::inner::expand_flat_file;
use bevy_auto_plugin_shared::util::TargetRequirePath;
use bevy_auto_plugin_shared::{AddSystemParams, flat_file};
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::{Item, Path, Token};

/// Attaches to a function accepting `&mut bevy::prelude::App`, automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    expand_flat_file(attr.into(), input.into())
        .unwrap_or_else(to_compile_error)
        .into()
}

fn flat_file_handle_attribute(
    attr: CompilerStream,
    input: CompilerStream,
    target: TargetRequirePath,
) -> CompilerStream {
    let cloned_input = input.clone();
    let parsed_item = parse_macro_input!(input as Item);
    let args = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr with Punctuated::<Path, Token![,]>::parse_terminated))
    };

    flat_file::inner::handle_attribute_outer(parsed_item, Span::call_site(), target, args)
        .map(|_| cloned_input)
        .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Automatically registers a type with the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RegisterTypes)
}
/// Automatically adds an event type to the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::AddEvents)
}
/// Automatically initializes a resource in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::InitResources)
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RequiredComponentAutoName)
}

/// Automatically initializes a State in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::InitStates)
}

/// Automatically registers a State type in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RegisterStateTypes)
}

/// Automatically add_system in the Bevy `App`.
#[proc_macro_attribute]
pub fn flat_file_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let args: AddSystemParams = match syn::parse(attr) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let cloned_input = input.clone();
    let item = parse_macro_input!(input as Item);
    flat_file::inner::handle_add_system_attribute_outer(item, args, Span::call_site())
        .map(|_| cloned_input)
        .unwrap_or_else(|err| err.to_compile_error().into())
}
