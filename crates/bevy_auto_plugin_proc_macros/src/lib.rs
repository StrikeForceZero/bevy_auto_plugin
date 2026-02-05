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

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/derives/AutoPlugin.md"))]
#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
pub fn derive_auto_plugin(input: CompilerStream) -> CompilerStream {
    expand::derive::auto_plugin::expand_derive_auto_plugin(input.into()).into()
}

#[allow(unused_variables, unused_mut, unreachable_code)]
#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/auto_plugin.md"))]
#[proc_macro_attribute]
pub fn auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin::expand_auto_plugin, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_register_type.md"))]
#[proc_macro_attribute]
pub fn auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_type, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_add_message.md"))]
#[proc_macro_attribute]
pub fn auto_add_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_message, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_configure_system_set.md"))]
#[proc_macro_attribute]
pub fn auto_configure_system_set(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_configure_system_set, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_init_resource.md"))]
#[proc_macro_attribute]
pub fn auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_resource, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_insert_resource.md"))]
#[proc_macro_attribute]
pub fn auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_insert_resource, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_init_state.md"))]
#[proc_macro_attribute]
pub fn auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_state, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_init_sub_state.md"))]
#[proc_macro_attribute]
pub fn auto_init_sub_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_init_sub_state, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_name.md"))]
#[proc_macro_attribute]
pub fn auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_name, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_register_state_type.md"))]
#[proc_macro_attribute]
pub fn auto_register_state_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_register_state_type, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_add_system.md"))]
#[proc_macro_attribute]
pub fn auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_system, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_add_observer.md"))]
#[proc_macro_attribute]
pub fn auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_observer, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_add_plugin.md"))]
#[proc_macro_attribute]
pub fn auto_add_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_add_plugin, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_component.md"))]
#[proc_macro_attribute]
pub fn auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_component, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_resource.md"))]
#[proc_macro_attribute]
pub fn auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_resource, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_event.md"))]
#[proc_macro_attribute]
pub fn auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_event, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_message.md"))]
#[proc_macro_attribute]
pub fn auto_message(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_message, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_states.md"))]
#[proc_macro_attribute]
pub fn auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_states, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_sub_states.md"))]
#[proc_macro_attribute]
pub fn auto_sub_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_sub_states, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_system.md"))]
#[proc_macro_attribute]
pub fn auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_system, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/rewrites/auto_observer.md"))]
#[proc_macro_attribute]
pub fn auto_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_observer, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_run_on_build.md"))]
#[proc_macro_attribute]
pub fn auto_run_on_build(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_run_on_build, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_bind_plugin.md"))]
#[proc_macro_attribute]
pub fn auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_bind_plugin::auto_bind_plugin_outer, attr, input)
}

#[doc = include_str!(concat!(env!("OUT_DIR"), "/docs/proc_attributes/actions/auto_plugin_build_hook.md"))]
#[proc_macro_attribute]
pub fn auto_plugin_build_hook(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    handle_attribute(expand::attr::auto_plugin_build_hook, attr, input)
}
