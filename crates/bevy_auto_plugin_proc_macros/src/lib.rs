use bevy_auto_plugin_shared::module::inner::expand_module;
use proc_macro::TokenStream as CompilerStream;
use proc_macro2::{Ident, TokenStream as MacroStream};
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

use bevy_auto_plugin_shared::attribute_args::{
    AddSystemArgs, GlobalAddSystemArgs, GlobalAutoPluginDeriveArgs,
    GlobalAutoPluginFnAttributeArgs, GlobalMacroArgs, GlobalStructOrEnumAttributeArgs,
    StructOrEnumAttributeArgs,
};
use bevy_auto_plugin_shared::flat_file::inner::expand_flat_file;
use bevy_auto_plugin_shared::global::__internal::_plugin_entry_block;
use bevy_auto_plugin_shared::util::TargetRequirePath;
use bevy_auto_plugin_shared::{default_app_ident, flat_file, ok_or_return_compiler_error};
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

    #[cfg(feature = "legacy_path_param")]
    let attr_cloned = attr.clone();

    let args: syn::Result<StructOrEnumAttributeArgs> = match syn::parse(attr) {
        Ok(args) => Ok(args),
        Err(err) => {
            #[cfg(not(feature = "legacy_path_param"))]
            {
                return err.to_compile_error().into();
            }
            #[cfg(feature = "legacy_path_param")]
            {
                bevy_auto_plugin_shared::util::StructOrEnumRef::try_from(&parsed_item)
                    .and_then(|se_ref| {
                        bevy_auto_plugin_shared::util::legacy_generics_from_path(
                            &se_ref,
                            attr_cloned.into(),
                        )
                    })
                    .map(|generics| StructOrEnumAttributeArgs { generics })
                    .map_err(|legacy_err| {
                        Error::new(err.span(), format!("new: {err}\nlegacy: {legacy_err}"))
                    })
            }
        }
    };
    let args = ok_or_return_compiler_error!(args);

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
    let args = parse_macro_input!(attr as AddSystemArgs);
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
    let params = match GlobalAutoPluginDeriveArgs::from_derive_input(&derive_input) {
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
    let params = match syn::parse::<GlobalAutoPluginFnAttributeArgs>(input_cloned) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error().into(),
    };
    let mut output = quote! {
        #item
    };
    todo!();
    output.into()
}

fn require_fn(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only functions and enum can use this attribute macro",
        )),
    }
}

fn require_struct_or_enum(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only struct and enum can use this attribute macro",
        )),
    }
}

fn global_attribute_inner<A, F>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    require: fn(&Item) -> syn::Result<&Ident>,
    parse_attr: fn(MacroStream) -> syn::Result<A>,
    body: F,
) -> MacroStream
where
    A: GlobalMacroArgs,
    F: FnOnce(&Ident, A, &Item) -> syn::Result<MacroStream>,
{
    let attr = attr.into();
    let input = input.into();

    let item: Item = ok_or_return_compiler_error!(syn::parse2(input.clone()));

    let ident = ok_or_return_compiler_error!(require(&item));

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote!( #item #output )
}

fn global_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    prefix: &'static str,
    require: fn(&Item) -> syn::Result<&Ident>,
    generate_fn: impl FnOnce(&Ident, <T as GlobalMacroArgs>::Input) -> syn::Result<MacroStream>,
) -> MacroStream
where
    T: GlobalMacroArgs,
{
    global_attribute_inner(
        attr,
        input,
        require,
        syn::parse2::<T>,
        |ident, params, _item| {
            let unique_ident = params.get_unique_ident(prefix, ident);
            let plugin = params.plugin().clone();
            let input = params.to_input(ident);
            let app_ident = default_app_ident();
            let register = generate_fn(&app_ident, input)?;
            let expr: syn::ExprClosure = syn::parse_quote!(|#app_ident| { #register });
            Ok(_plugin_entry_block(&unique_ident, &plugin, &expr))
        },
    )
}

#[proc_macro_attribute]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_type_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_register_type,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_add_event_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_add_event,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_resource_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_init_resource,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_state_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_init_state,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_name_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_auto_name,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_state_type_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_register_state_type,
    )
    .into()
}

#[proc_macro_attribute]
pub fn global_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    global_attribute_outer::<GlobalAddSystemArgs>(
        attr,
        input,
        "_global_plugin_add_system_",
        require_fn,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_add_system,
    )
    .into()
}
