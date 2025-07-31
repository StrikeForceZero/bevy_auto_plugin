use bevy_auto_plugin_shared::module::inner::expand_module;
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;
use syn::{Error, ItemFn, Path, parse_macro_input, parse_str};

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
use bevy_auto_plugin_shared::global::__internal::_plugin_entry_block;
use bevy_auto_plugin_shared::util::TargetRequirePath;
use bevy_auto_plugin_shared::{
    AddSystemParams, GlobalAutoPluginDeriveParams, GlobalAutoPluginFnAttributeParams,
    GlobalStructOrEnumAttributeParams, StructOrEnumAttributeParams, default_app_ident, flat_file,
    generate_register_type, generate_register_types,
    get_unique_ident_for_global_struct_or_enum_attribute,
};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::Item;

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
    let args = parse_macro_input!(attr as StructOrEnumAttributeParams);

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
    let cloned_input = input.clone();
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(attr as AddSystemParams);
    flat_file::inner::handle_add_system_attribute_outer(item, args, Span::call_site())
        .map(|_| cloned_input)
        .unwrap_or_else(|err| err.to_compile_error().into())
}

/* global */

#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
pub fn derive_global_auto_plugin(input: CompilerStream) -> CompilerStream {
    use darling::FromDeriveInput;
    use syn::DeriveInput;

    let derive_input = parse_macro_input!(input as DeriveInput);
    let params = match GlobalAutoPluginDeriveParams::from_derive_input(&derive_input) {
        Ok(params) => params,
        Err(err) => return err.write_errors().into(),
    };
    let ident = &params.ident; // `Test`
    let generics = &params.generics; // `<T1, T2>`
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = MacroStream::new();

    output.extend(quote! {
        impl #impl_generics ::bevy_auto_plugin_shared::global::__internal::AutoPluginTypeId
            for #ident #ty_generics #where_clause
        {
            fn type_id() -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }
        }
    });

    if params.auto_plugin.impl_plugin_trait {
        let full_names = if params.auto_plugin.generics.is_empty() {
            vec![ident.to_string()]
        } else {
            params
                .auto_plugin
                .generics
                .iter()
                .map(|tl| format!("{}::<{}>", ident, tl.to_token_stream()))
                .collect()
        };
        for full_name in full_names {
            let path_with_generics = match parse_str::<Path>(&full_name) {
                Ok(p) => p,
                Err(err) => return err.into_compile_error().into(),
            };

            output.extend(quote! {
                impl ::bevy_auto_plugin_shared::global::__internal::bevy_app::Plugin for #path_with_generics {
                    fn build(&self, app: &mut ::bevy_auto_plugin_shared::global::__internal::bevy_app::App) {
                        <Self as ::bevy_auto_plugin_shared::global::__internal::AutoPlugin>::build(self, app);
                    }
                }

                impl ::bevy_auto_plugin_shared::global::__internal::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if params.auto_plugin.impl_generic_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin_shared::global::__internal::bevy_app::Plugin
                for #ident #ty_generics #where_clause
            {
                fn build(&self, app: &mut ::bevy_auto_plugin_shared::global::__internal::bevy_app::App) {
                    <Self as ::bevy_auto_plugin_shared::global::__internal::AutoPlugin>::build(self, app);
                }
            }
        });
    }

    if params.auto_plugin.impl_generic_auto_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin_shared::global::__internal::AutoPlugin
                for #ident #ty_generics #where_clause
            {}
        });
    }

    output.into()
}

#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
pub fn global_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let input_cloned = input.clone();
    let item = parse_macro_input!(input as ItemFn);
    let params = match syn::parse::<GlobalAutoPluginFnAttributeParams>(input_cloned) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error().into(),
    };
    let mut output = quote! {
        #item
    };
    todo!();
    output.into()
}

#[proc_macro_attribute]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    let item = parse_macro_input!(input as Item);
    let ident = match &item {
        Item::Struct(item_struct) => &item_struct.ident,
        Item::Enum(item_enum) => &item_enum.ident,
        _ => {
            return Error::new(Span::call_site(), "Only struct and enum can be registered")
                .into_compile_error()
                .into();
        }
    };
    let params = match syn::parse::<GlobalStructOrEnumAttributeParams>(attr) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error().into(),
    };
    let unique_ident = get_unique_ident_for_global_struct_or_enum_attribute(
        "_global_plugin_register_type_",
        ident,
        &params,
    );
    let generics = &params.inner.generics;
    let target = quote! {
        #ident::<#generics>
    };
    let app_ident = default_app_ident();
    let register_tokens = match generate_register_type(&app_ident, target.to_string()) {
        Ok(tokens) => tokens,
        Err(err) => return err.into_compile_error().into(),
    };
    let expr = syn::parse_quote! { |#app_ident| { #register_tokens } };
    let registration = _plugin_entry_block(&unique_ident, &params.plugin, &expr);
    let output = quote! {
        #item
        #registration
    };
    output.into()
}
