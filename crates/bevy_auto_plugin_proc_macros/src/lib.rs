use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

#[allow(dead_code)]
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
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(module::inner::expand_module, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/module/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_register_type(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds a message type to the Bevy `App`.
#[doc = include_str!("docs/module/auto_add_message.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_add_message(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a resource in the Bevy `App`.
#[doc = include_str!("docs/module/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_init_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("docs/module/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_insert_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[doc = include_str!("docs/module/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_name(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/module/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_init_state(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically registers a `State<T>` and `NextState<T>` in the Bevy `App`.
#[doc = include_str!("docs/module/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
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
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn module_auto_add_system(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds an observer to the Bevy `App`.
#[doc = include_str!("docs/module/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
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
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::expand_flat_file, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::handle_register_type_attribute,
        attr,
        input,
    )
}

/// Automatically adds a message type to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_message.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_add_event_attribute, attr, input)
}

/// Automatically initializes a resource in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        flat_file::inner::handle_init_resource_attribute,
        attr,
        input,
    )
}

/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_auto_name_attribute, attr, input)
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_init_state_attribute, attr, input)
}

/// Automatically registers a State type in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handle_attribute(
        flat_file::inner::handle_register_state_type_attribute,
        attr,
        input,
    )
}

/// Automatically add_system in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_add_system_attribute, attr, input)
}

/// Automatically adds an observer to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::handle_add_observer_attribute, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
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

/// Automatically registers item as Component for bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_component.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_component, attr, input)
}

/// Automatically registers item as Resource for bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_resource, attr, input)
}

/// Automatically registers item as Event for bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_event, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_states.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_states, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_system, attr, input)
}

/// Automatically adds flat_file observer to bevy app. (See below for additional options)
#[doc = include_str!("docs/flat_file/shorthand/auto_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
#[cfg_attr(
    not(feature = "ignore_flat_file_or_module_deprecation"),
    deprecated(
        since = "0.5.0",
        note = "See https://github.com/StrikeForceZero/bevy_auto_plugin/issues/19"
    )
)]
pub fn flat_file_auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(flat_file::inner::flat_file_auto_observer, attr, input)
}

/* global */

#[cfg(feature = "mode_global")]
use bevy_auto_plugin_shared::__private::modes::global;

/// Derives `AutoPlugin` which generates the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("docs/global/derive_auto_plugin.md")]
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
#[cfg(feature = "mode_global")]
pub fn derive_global_auto_plugin(input: CompilerStream) -> CompilerStream {
    global::inner::expand_global_derive_global_auto_plugin(input.into()).into()
}

/// Attaches to a fn and injects a call to the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("docs/global/auto_plugin.md")]
#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::expand_global_auto_plugin, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/global/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_register_type_outer, attr, input)
}

/// Automatically adds a message type to the Bevy `App`.
#[doc = include_str!("docs/global/auto_add_message.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_message_outer, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("docs/global/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_init_resource_outer, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
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

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/global/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_init_state_outer, attr, input)
}

/// Automatically registers a required component `Name` with a value using the concrete name of the item.
#[doc = include_str!("docs/global/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_name_outer, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
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

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_system_outer, attr, input)
}

/// Automatically adds the fn as a global observer to bevy app. (See below for additional options)
#[doc = include_str!("docs/global/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_add_observer_outer, attr, input)
}

/// Automatically registers item as Component for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_component.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_component, attr, input)
}

/// Automatically registers item as Resource for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_resource, attr, input)
}

/// Automatically registers item as Event for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_event, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_states.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_states, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_system, attr, input)
}

/// Automatically adds global observer to bevy app. (See below for additional options)
#[doc = include_str!("docs/global/shorthand/auto_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_observer, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[doc = include_str!("docs/global/shorthand/auto_bind_plugin.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(global::inner::global_auto_bind_plugin_outer, attr, input)
}
