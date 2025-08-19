#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::__private::attribute_args::ItemAttributeArgs;
#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::__private::attribute_args::attributes::prelude::{
    AddEventAttributeArgs, AutoNameAttributeArgs, InitResourceAttributeArgs,
    InitStateAttributeArgs, RegisterStateTypeAttributeArgs, RegisterTypeAttributeArgs,
};
#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::__private::context::{
    AutoPluginContextInsert, SupportsAutoPluginContextInsert, ToTokenStringValue,
};
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
/* Module */

#[cfg(feature = "mode_module")]
use bevy_auto_plugin_shared::__private::modes::module;

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
#[doc = include_str!("docs/module/auto_plugin.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    module::inner::expand_module(attr.into(), input.into()).into()
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
    flat_file::inner::expand_flat_file(attr.into(), input.into()).into()
}

/// thin adapter converting between the compiler-level and proc_macro2 streams
#[cfg(feature = "mode_flat_file")]
fn flat_file_handle_attribute<A>(attr: CompilerStream, input: CompilerStream) -> CompilerStream
where
    A: ItemAttributeArgs + SupportsAutoPluginContextInsert,
    ToTokenStringValue<A>: AutoPluginContextInsert,
{
    flat_file::inner::flat_file_handle_attribute::<A>(attr.into(), input.into()).into()
}

/// Automatically registers a type with the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute::<RegisterTypeAttributeArgs>(attr, input)
}
/// Automatically adds an event type to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute::<AddEventAttributeArgs>(attr, input)
}
/// Automatically initializes a resource in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute::<InitResourceAttributeArgs>(attr, input)
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute::<AutoNameAttributeArgs>(attr, input)
}

/// Automatically initializes a State in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute::<InitStateAttributeArgs>(attr, input)
}

/// Automatically registers a State type in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    flat_file_handle_attribute::<RegisterStateTypeAttributeArgs>(attr, input)
}

/// Automatically add_system in the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file::inner::handle_add_system_attribute(attr.into(), input.into()).into()
}

/// Automatically adds an observer to the Bevy `App`.
#[doc = include_str!("docs/flat_file/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file::inner::handle_add_observer_attribute(attr.into(), input.into()).into()
}

#[doc = include_str!("docs/flat_file/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_insert_resource(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    flat_file::inner::handle_insert_resource_attribute(attr.into(), input.into()).into()
}

/* global */

#[cfg(feature = "mode_global")]
use bevy_auto_plugin_shared::__private::modes::global;

/// thin adapter converting between the compiler-level and proc_macro2 streams
#[cfg(feature = "mode_global")]
fn global_handle_attribute<F: Fn(MacroStream, MacroStream) -> MacroStream>(
    handler: F,
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    handler(attr.into(), input.into()).into()
}

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
    global_handle_attribute(global::inner::expand_global_auto_plugin, attr, input)
}

#[doc = include_str!("docs/global/auto_register_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_register_type_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_add_event.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_add_event_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_init_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_init_resource_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_insert_resource.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(
        global::inner::global_auto_insert_resource_outer,
        attr,
        input,
    )
}

#[doc = include_str!("docs/global/auto_init_state.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_init_state_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_name.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_name_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_register_state_type.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    global_handle_attribute(
        global::inner::global_auto_register_state_type_outer,
        attr,
        input,
    )
}

#[doc = include_str!("docs/global/auto_add_system.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_add_system_outer, attr, input)
}

#[doc = include_str!("docs/global/auto_add_observer.md")]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_observer(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_handle_attribute(global::inner::global_auto_add_observer_outer, attr, input)
}

#[proc_macro_attribute]
pub fn global_auto_component(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::Mode;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::component::ComponentAttributeArgs;
    use syn::parse_macro_input;
    let args = parse_macro_input!(attr as GlobalArgs<ComponentAttributeArgs>);
    let args_ts = args
        .inner
        .expand_attrs(&Mode::Global {
            plugin: args.plugin,
        })
        .to_token_stream();
    let input = proc_macro2::TokenStream::from(input);
    CompilerStream::from(quote! {
        #args_ts
        #input
    })
}

#[proc_macro_attribute]
pub fn global_auto_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::Mode;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::resource::ResourceAttributeArgs;
    use syn::parse_macro_input;
    let args = parse_macro_input!(attr as GlobalArgs<ResourceAttributeArgs>);
    let args_ts = args
        .inner
        .expand_attrs(&Mode::Global {
            plugin: args.plugin,
        })
        .to_token_stream();
    let input = proc_macro2::TokenStream::from(input);
    CompilerStream::from(quote! {
        #args_ts
        #input
    })
}

#[proc_macro_attribute]
pub fn global_auto_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::Mode;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::event::EventAttributeArgs;
    use syn::parse_macro_input;
    let args = parse_macro_input!(attr as GlobalArgs<EventAttributeArgs>);
    let args_ts = args
        .inner
        .expand_attrs(&Mode::Global {
            plugin: args.plugin,
        })
        .to_token_stream();
    let input = proc_macro2::TokenStream::from(input);
    CompilerStream::from(quote! {
        #args_ts
        #input
    })
}

#[proc_macro_attribute]
pub fn global_auto_states(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::Mode;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::states::StatesAttributeArgs;
    use syn::parse_macro_input;
    let args = parse_macro_input!(attr as GlobalArgs<StatesAttributeArgs>);
    let args_ts = args
        .inner
        .expand_attrs(&Mode::Global {
            plugin: args.plugin,
        })
        .to_token_stream();
    let input = proc_macro2::TokenStream::from(input);
    CompilerStream::from(quote! {
        #args_ts
        #input
    })
}

#[proc_macro_attribute]
pub fn global_auto_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::Mode;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
    use bevy_auto_plugin_shared::__private::attribute_args::attributes::shorthand::system::SystemAttributeArgs;
    use syn::parse_macro_input;
    let args = parse_macro_input!(attr as GlobalArgs<SystemAttributeArgs>);
    let args_ts = args
        .inner
        .expand_attrs(&Mode::Global {
            plugin: args.plugin,
        })
        .to_token_stream();
    let input = proc_macro2::TokenStream::from(input);
    CompilerStream::from(quote! {
        #args_ts
        #input
    })
}

#[proc_macro_attribute]
pub fn global_auto_bind_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::__private::attribute_args::GlobalArgs;
    use proc_macro2::Span;
    use quote::quote;
    use std::mem;
    use syn::parse::Parser as _;
    use syn::{Attribute, Item, Meta, MetaList, Token, parse_macro_input, punctuated::Punctuated};

    // Parse the item and the macro's own args (we only need `plugin`)
    let mut item: Item = parse_macro_input!(input as Item);
    let args = parse_macro_input!(attr as GlobalArgs<()>);
    let plugin = args.plugin;

    // Helper: take attrs out of the item so we can transform & put them back.
    fn take_attrs(item: &mut Item) -> Option<Vec<Attribute>> {
        match item {
            Item::Fn(f) => Some(mem::take(&mut f.attrs)),
            Item::Struct(s) => Some(mem::take(&mut s.attrs)),
            Item::Enum(e) => Some(mem::take(&mut e.attrs)),
            _ => None,
        }
    }
    fn put_attrs(item: &mut Item, attrs: Vec<Attribute>) {
        match item {
            Item::Fn(f) => f.attrs = attrs,
            Item::Struct(s) => s.attrs = attrs,
            Item::Enum(e) => e.attrs = attrs,
            _ => {}
        }
    }

    // Only functions/structs/enums are supported
    let Some(orig_attrs) = take_attrs(&mut item) else {
        return syn::Error::new(
            Span::call_site(),
            "auto_bind_plugin supports only functions, structs, or enums",
        )
        .into_compile_error()
        .into();
    };

    // Detect whether a MetaList already has a `plugin` key
    fn list_has_key(ml: &MetaList, key: &str) -> bool {
        // Parse the inside of (...) as a comma-separated list of Meta
        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        match parser.parse2(ml.tokens.clone()) {
            Ok(list) => list.iter().any(|m| match m {
                Meta::NameValue(nv) => nv.path.is_ident(key),
                Meta::List(ml2) => ml2.path.is_ident(key),
                Meta::Path(p) => p.is_ident(key),
            }),
            Err(_) => false, // If we can't parse, assume missing and try to inject
        }
    }

    // Transform attributes: inject `plugin = #plugin` where needed
    let mut new_attrs = Vec::with_capacity(orig_attrs.len());
    for attr in orig_attrs {
        let last = attr
            .path()
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();

        // Only touch attributes like #[auto_*]
        if !last.starts_with("auto_") {
            new_attrs.push(attr);
            continue;
        }

        // If it already has a plugin arg, keep it as-is
        let already_has_plugin = match &attr.meta {
            Meta::List(ml) => list_has_key(ml, "plugin"),
            Meta::Path(_) => false,
            Meta::NameValue(_) => true, // odd form like #[auto_x = ...]; leave it alone
        };
        if already_has_plugin {
            new_attrs.push(attr);
            continue;
        }

        // Inject `plugin = #plugin` (preserving existing args when present)
        match &attr.meta {
            Meta::Path(p) => {
                // #[auto_x] -> #[auto_x(plugin = #plugin)]
                let p = p.clone();
                let injected: Attribute = syn::parse_quote!( #[#p(plugin = #plugin)] );
                new_attrs.push(injected);
            }
            Meta::List(ml) => {
                // #[auto_x(...)] -> #[auto_x(plugin = #plugin, ...)]
                let path = ml.path.clone();
                let inner = ml.tokens.clone();
                let injected: Attribute = if inner.is_empty() {
                    syn::parse_quote!( #[#path(plugin = #plugin)] )
                } else {
                    syn::parse_quote!( #[#path(plugin = #plugin, #inner)] )
                };
                new_attrs.push(injected);
            }
            Meta::NameValue(_) => {
                // Leave uncommon #[auto_x = ...] untouched
                new_attrs.push(attr);
            }
        }
    }

    put_attrs(&mut item, new_attrs);

    // Re-emit the modified item
    CompilerStream::from(quote! { #item })
}
