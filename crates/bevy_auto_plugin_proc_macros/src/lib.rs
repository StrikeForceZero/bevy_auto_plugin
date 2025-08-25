#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::__private::attribute_args::attributes::prelude::{
    AddEventAttributeArgs, AutoNameAttributeArgs, InitResourceAttributeArgs,
    InitStateAttributeArgs, RegisterStateTypeAttributeArgs, RegisterTypeAttributeArgs,
};
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

/// thin adapter converting between the compiler-level and proc_macro2 streams
fn handle_attribute<F: Fn(MacroStream, MacroStream) -> MacroStream>(
    handler: F,
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handler(attr.into(), input.into()).into()
}

/* Module */

#[cfg(feature = "mode_module")]
use bevy_auto_plugin_shared::__private::modes::module;

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("docs/module/auto_plugin.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(module::inner::expand_module, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/module/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_register_type(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds an event type to the Bevy `App`.
#[doc = include_str!("docs/module/auto_add_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_add_event(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a resource in the Bevy `App`.
#[doc = include_str!("docs/module/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_init_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("docs/module/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_insert_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[doc = include_str!("docs/module/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_name(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/module/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_init_state(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically registers a `State<T>` and `NextState<T>` in the Bevy `App`.
#[doc = include_str!("docs/module/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_register_state_type(
    _attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically add_system in the Bevy `App`.
#[doc = include_str!("docs/module/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_add_system(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds an observer to the Bevy `App`.
#[doc = include_str!("docs/module/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_add_observer(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/* Flat File */
#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::__private::modes::flat_file;

/// Attaches to a function accepting `&mut bevy::prelude::App`, automatically registering types, events, and resources in the `App`.
#[doc = include_str!("docs/flat_file/auto_plugin.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::expand_flat_file, attr, input)
}
/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<RegisterTypeAttributeArgs>,
        attr,
        input,
    )
}
/// Automatically adds an event type to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<AddEventAttributeArgs>,
        attr,
        input,
    )
}
/// Automatically initializes a resource in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<InitResourceAttributeArgs>,
        attr,
        input,
    )
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<AutoNameAttributeArgs>,
        attr,
        input,
    )
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<InitStateAttributeArgs>,
        attr,
        input,
    )
}

/// Automatically registers a State type in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handle_attribute(
        flat_file::inner::flat_file_handle_attribute::<RegisterStateTypeAttributeArgs>,
        attr,
        input,
    )
}

/// Automatically add_system in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_add_system_attribute, attr, input)
}

/// Automatically adds an observer to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_add_observer_attribute, attr, input)
}

#[doc = include_str!("docs/flat_file/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_insert_resource(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handle_attribute(
        flat_file::inner::handle_insert_resource_attribute,
        attr,
        input,
    )
}

/* global */

#[cfg(feature = "mode_global")]
use bevy_auto_plugin_shared::__private::modes::global;

#[doc = include_str!("docs/global/derive_auto_plugin.md")]
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
#[cfg(feature = "mode_global")]
pub fn derive_global_auto_plugin(input: CompilerStream) -> CompilerStream {
    global::inner::expand_global_derive_global_auto_plugin(input.into()).into()
}

#[doc = include_str!("docs/global/auto_plugin.md")]
#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::expand_global_auto_plugin, attr, input)
}

#[doc = include_str!("docs/global/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_register_type_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_add_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_event_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_init_resource_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        global::inner::global_auto_insert_resource_outer,
        attr,
        input,
    )
}

#[doc = include_str!("docs/global/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_init_state_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_name_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handle_attribute(
        global::inner::global_auto_register_state_type_outer,
        attr,
        input,
    )
}

#[doc = include_str!("docs/global/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_system_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_observer_outer, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_component.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_component, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_resource, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_event, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_states.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_states, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_system, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_observer, attr, input)
}

#[doc = include_str!("docs/global/shorthand/auto_bind_plugin.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_bind_plugin_outer, attr, input)
}
