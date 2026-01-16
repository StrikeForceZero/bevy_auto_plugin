use bevy_auto_plugin_shared::__private::expand;
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

/// Thin adapter converting between the compiler-level and proc_macro2 streams
fn handle_attribute<F: Fn(MacroStream, MacroStream) -> MacroStream>(
    handler: F,
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handler(attr.into(), input.into()).into()
}

/// Derives `AutoPlugin`, which generates the initialization function that automatically registers types, events, resources, system, ..., etc. in the `App`.
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
pub fn derive_auto_plugin(input: CompilerStream) -> CompilerStream {
    expand::derive::auto_plugin::expand_derive_auto_plugin(input.into()).into()
}

/// Attaches to an `fn` and injects a call to the `AutoPlugin` initialization function that automatically registers types, events, resources, system, ..., etc. in the `App`.
#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
pub fn auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin::expand_auto_plugin, attr, input)
}

/// Automatically registers item deriving `Reflect` in `TypeRegistry` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_type, attr, input)
}

/// Automatically adds a `Message` to the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_message, attr, input)
}

/// Automatically configures a `SystemSet` in the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_configure_system_set(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_configure_system_set, attr, input)
}

/// Automatically initializes a `Resource` in the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_resource, attr, input)
}

/// Automatically inserts a `Resource` in the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_insert_resource, attr, input)
}

/// Automatically initializes a `State` in the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_state, attr, input)
}

/// Automatically initializes a `SubState` in the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_init_sub_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_sub_state, attr, input)
}

/// Automatically registers a required component `Name` with a value using the concrete name of the item.
#[proc_macro_attribute]
pub fn auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_name, attr, input)
}

/// Automatically registers item representing `States` in `TypeRegistry` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_state_type, attr, input)
}

/// Automatically adds the `fn` as a `System` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_system, attr, input)
}

/// Automatically adds the `fn` as a `Observer` to the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_observer, attr, input)
}

/// Automatically adds the `Plugin` as a sub-plugin to the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_add_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_plugin, attr, input)
}

/// Automatically registers item as `Component` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_component, attr, input)
}

/// Automatically registers item as `Resource` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_resource, attr, input)
}

/// Automatically registers item as `Event` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_event, attr, input)
}

/// Automatically registers item as `Message` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_message, attr, input)
}

/// Automatically registers item as `States` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_states, attr, input)
}

/// Automatically registers item as `SubStates` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_sub_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_sub_states, attr, input)
}

/// Automatically adds the `fn` as a `System` for the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_system, attr, input)
}

/// Automatically adds `Observer` to the Bevy `App`.
#[proc_macro_attribute]
pub fn auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_observer, attr, input)
}

/// Automatically runs the `fn` on build.
#[proc_macro_attribute]
pub fn auto_run_on_build(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_run_on_build, attr, input)
}

/// Automatically binds `plugin = _` to every auto_* attribute below it
#[proc_macro_attribute]
pub fn auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_bind_plugin::auto_bind_plugin_outer, attr, input)
}

#[proc_macro_attribute]
pub fn auto_plugin_custom(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin_custom, attr, input)
}
