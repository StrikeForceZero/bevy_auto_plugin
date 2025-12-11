use bevy_auto_plugin_shared::__private::expand;
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

/// Derives `AutoPlugin` which generates the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("../docs/proc_attributes/derive_auto_plugin.md")]
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
pub fn derive_auto_plugin(input: CompilerStream) -> CompilerStream {
    expand::derive::auto_plugin::expand_derive_auto_plugin(input.into()).into()
}

/// Attaches to a fn and injects a call to the initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("../docs/proc_attributes/auto_plugin.md")]
#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
pub fn auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin::expand_auto_plugin, attr, input)
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_register_type.md")]
#[proc_macro_attribute]
pub fn auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_type, attr, input)
}

/// Automatically adds a message type to the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_add_message.md")]
#[proc_macro_attribute]
pub fn auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_message, attr, input)
}

/// Automatically configures a `SystemSet`.
#[doc = include_str!("../docs/proc_attributes/auto_configure_system_set.md")]
#[proc_macro_attribute]
pub fn auto_configure_system_set(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_configure_system_set, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_init_resource.md")]
#[proc_macro_attribute]
pub fn auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_resource, attr, input)
}

/// Automatically inserts a resource in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_insert_resource.md")]
#[proc_macro_attribute]
pub fn auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_insert_resource, attr, input)
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_init_state.md")]
#[proc_macro_attribute]
pub fn auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_state, attr, input)
}

/// Automatically initializes a SubState in the Bevy `App`.
#[doc = include_str!("../docs/proc_attributes/auto_init_sub_state.md")]
#[proc_macro_attribute]
pub fn auto_init_sub_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_sub_state, attr, input)
}

/// Automatically registers a required component `Name` with a value using the concrete name of the item.
#[doc = include_str!("../docs/proc_attributes/auto_name.md")]
#[proc_macro_attribute]
pub fn auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_name, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_register_state_type.md")]
#[proc_macro_attribute]
pub fn auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_state_type, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_add_system.md")]
#[proc_macro_attribute]
pub fn auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_system, attr, input)
}

/// Automatically adds the fn as a proc_attributes observer to bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_add_observer.md")]
#[proc_macro_attribute]
pub fn auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_observer, attr, input)
}

/// Automatically adds the plugin as a sub-plugin to bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_add_system.md")]
#[proc_macro_attribute]
pub fn auto_add_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_plugin, attr, input)
}

/// Automatically registers item as Component for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_component.md")]
#[proc_macro_attribute]
pub fn auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_component, attr, input)
}

/// Automatically registers item as Resource for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_resource.md")]
#[proc_macro_attribute]
pub fn auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_resource, attr, input)
}

/// Automatically registers item as Event for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_event.md")]
#[proc_macro_attribute]
pub fn auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_event, attr, input)
}

/// Automatically registers item as Message for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_message.md")]
#[proc_macro_attribute]
pub fn auto_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_message, attr, input)
}

/// Automatically registers item as States for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_states.md")]
#[proc_macro_attribute]
pub fn auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_states, attr, input)
}

/// Automatically registers item as SubStates for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_sub_states.md")]
#[proc_macro_attribute]
pub fn auto_sub_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_sub_states, attr, input)
}

/// Automatically adds the fn as a system for bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_system.md")]
#[proc_macro_attribute]
pub fn auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_system, attr, input)
}

/// Automatically adds proc_attributes observer to bevy app. (See below for additional options)
#[doc = include_str!("../docs/proc_attributes/auto_observer.md")]
#[proc_macro_attribute]
pub fn auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_observer, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[doc = include_str!("../docs/proc_attributes/auto_run_on_build.md")]
#[proc_macro_attribute]
pub fn auto_run_on_build(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_run_on_build, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[doc = include_str!("../docs/proc_attributes/auto_bind_plugin.md")]
#[proc_macro_attribute]
pub fn auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(
        expand::attr::auto_bind_plugin::auto_bind_plugin_outer,
        attr,
        input,
    )
}
