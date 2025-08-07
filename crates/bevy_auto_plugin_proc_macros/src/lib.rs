use proc_macro::TokenStream as CompilerStream;
use proc_macro2::TokenStream as MacroStream;

#[cfg(any(feature = "mode_module", feature = "mode_flat_file"))]
fn to_compile_error(err: syn::Error) -> MacroStream {
    err.to_compile_error()
}

/* Module */

/// Attaches to a module and generates an initialization function that automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::modes::module::inner::expand_module;
    expand_module(attr.into(), input.into())
        .unwrap_or_else(to_compile_error)
        .into()
}

/// Automatically registers a type with the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_register_type(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically adds an event type to the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_add_event(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a resource in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_init_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically inserts a resource in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_insert_resource(_args: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_name(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically initializes a State in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_init_state(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/// Automatically registers a State<T> and NextState<T> in the Bevy `App`.
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
#[proc_macro_attribute]
#[cfg(feature = "mode_module")]
pub fn module_auto_add_system(_attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    // Just return the input unchanged; this acts as a marker.
    input
}

/* Flat File */
#[cfg(feature = "mode_flat_file")]
use bevy_auto_plugin_shared::{
    modes::flat_file, modes::flat_file::inner::expand_flat_file, util::TargetRequirePath,
};
/// Attaches to a function accepting `&mut bevy::prelude::App`, automatically registering types, events, and resources in the `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    expand_flat_file(attr.into(), input.into())
        .unwrap_or_else(to_compile_error)
        .into()
}

#[cfg(feature = "mode_flat_file")]
fn flat_file_handle_attribute(
    attr: CompilerStream,
    input: CompilerStream,
    target: TargetRequirePath,
) -> CompilerStream {
    use bevy_auto_plugin_shared::attribute_args::StructOrEnumAttributeArgs;
    use bevy_auto_plugin_shared::ok_or_return_compiler_error;
    use syn::parse_macro_input;
    let cloned_input = input.clone();
    let parsed_item = parse_macro_input!(input as syn::Item);

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
                use bevy_auto_plugin_shared::util::StructOrEnumRef;
                StructOrEnumRef::try_from(&parsed_item)
                    .and_then(|se_ref| {
                        bevy_auto_plugin_shared::util::legacy_generics_from_path(
                            &se_ref,
                            attr_cloned.into(),
                        )
                    })
                    .map(StructOrEnumAttributeArgs::from)
                    .map_err(|legacy_err| {
                        syn::Error::new(err.span(), format!("\nnew: {err}\nlegacy: {legacy_err}"))
                    })
            }
        }
    };
    let args = ok_or_return_compiler_error!(args);

    flat_file::inner::handle_attribute_outer(
        parsed_item,
        proc_macro2::Span::call_site(),
        target,
        args,
    )
    .map(|_| cloned_input)
    .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Automatically registers a type with the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RegisterTypes)
}
/// Automatically adds an event type to the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::AddEvents)
}
/// Automatically initializes a resource in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::InitResources)
}
/// Automatically associates a required component `Name` with the default value set to the ident in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RequiredComponentAutoName)
}

/// Automatically initializes a State in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::InitStates)
}

/// Automatically registers a State type in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    flat_file_handle_attribute(attr, input, TargetRequirePath::RegisterStateTypes)
}

/// Automatically add_system in the Bevy `App`.
#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::attribute_args::AddSystemArgs;
    use syn::parse_macro_input;
    let cloned_input = input.clone();
    let item = parse_macro_input!(input as syn::ItemFn);
    let args = parse_macro_input!(attr as AddSystemArgs);
    flat_file::inner::handle_add_system_attribute_outer(item, args, proc_macro2::Span::call_site())
        .map(|_| cloned_input)
        .unwrap_or_else(|err| err.to_compile_error().into())
}

#[proc_macro_attribute]
#[cfg(feature = "mode_flat_file")]
pub fn flat_file_auto_insert_resource(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    use bevy_auto_plugin_shared::attribute_args::InsertResourceArgs;
    use syn::parse_macro_input;
    let cloned_input = input.clone();
    let item = parse_macro_input!(input as syn::Item);
    // TODO: compiler error if multiple auto_insert_resource attributes found for same type
    let insert_resource_args = parse_macro_input!(attr as InsertResourceArgs);
    if let Err(err) = insert_resource_args.validate_resource() {
        return err.to_compile_error().into();
    }
    flat_file::inner::handle_insert_resource_outer(
        item,
        proc_macro2::Span::call_site(),
        insert_resource_args,
    )
    .map(|_| cloned_input)
    .unwrap_or_else(|err| err.to_compile_error().into())
}

/* global */

#[cfg(feature = "mode_global")]
use bevy_auto_plugin_shared::{
    attribute_args::{
        GlobalAddSystemArgs, GlobalAutoPluginDeriveArgs, GlobalAutoPluginFnAttributeArgs,
        GlobalInsertResourceAttributeArgs, GlobalStructOrEnumAttributeArgs,
    },
    modes::global::inner,
};

#[proc_macro_derive(AutoPlugin, attributes(auto_plugin))]
#[cfg(feature = "mode_global")]
pub fn derive_global_auto_plugin(input: CompilerStream) -> CompilerStream {
    use darling::FromDeriveInput;
    use quote::ToTokens;
    use quote::quote;
    use syn::DeriveInput;
    use syn::parse_macro_input;

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
        impl #impl_generics ::bevy_auto_plugin_shared::modes::global::__internal::AutoPluginTypeId
            for #ident #ty_generics #where_clause
        {
            fn type_id() -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }
        }
    });

    let mut auto_plugin_implemented = false;

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
            let path_with_generics = match syn::parse_str::<syn::Path>(&full_name) {
                Ok(p) => p,
                Err(err) => return err.into_compile_error().into(),
            };

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin_shared::modes::global::__internal::bevy_app::Plugin for #path_with_generics {
                    fn build(&self, app: &mut ::bevy_auto_plugin_shared::modes::global::__internal::bevy_app::App) {
                        <Self as ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin>::build(self, app);
                    }
                }

                impl ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if params.auto_plugin.impl_generic_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin_shared::modes::global::__internal::bevy_app::Plugin
                for #ident #ty_generics #where_clause
            {
                fn build(&self, app: &mut ::bevy_auto_plugin_shared::modes::global::__internal::bevy_app::App) {
                    <Self as ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin>::build(self, app);
                }
            }
        });
    }

    // TODO: maybe default to this behavior
    if params.auto_plugin.impl_generic_auto_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin
                for #ident #ty_generics #where_clause
            {}
        });
    } else if !auto_plugin_implemented {
        auto_plugin_implemented = true;

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
            let path_with_generics = match syn::parse_str::<syn::Path>(&full_name) {
                Ok(p) => p,
                Err(err) => return err.into_compile_error().into(),
            };

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if auto_plugin_implemented {
        // satisfy linter #[warn(unused_assignments)]
    }

    output.into()
}

#[allow(unused_variables, unused_mut, unreachable_code)]
#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_plugin(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::attribute_args::default_app_ident;
    use bevy_auto_plugin_shared::util::{
        FnParamMutabilityCheckErrMessages, is_fn_param_mutable_reference,
    };
    use quote::quote;
    use syn::parse_macro_input;
    use syn::spanned::Spanned;
    let item = parse_macro_input!(input as syn::ItemFn);
    let params = match syn::parse::<GlobalAutoPluginFnAttributeArgs>(attr) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error().into(),
    };
    let vis = &item.vis;
    let attrs = &item.attrs;
    let sig = &item.sig;
    let block = &item.block;
    let ident = &sig.ident;
    let generics = &sig.generics;
    let inputs = &sig.inputs;
    let self_args = inputs
        .into_iter()
        .flat_map(|input| match input {
            syn::FnArg::Receiver(recv) => Some(syn::Ident::new("self", recv.span())),
            syn::FnArg::Typed(_) => None,
        })
        .take(1)
        .collect::<Vec<_>>();
    let self_arg = self_args.first();

    let default_app_ident = default_app_ident();
    let app_param_ident = params.app_param.as_ref().unwrap_or(&default_app_ident);

    if let Err(err) = is_fn_param_mutable_reference(
        &item,
        app_param_ident,
        FnParamMutabilityCheckErrMessages {
            not_mutable_message: format!("bevy app param: {app_param_ident} is not mutable"),
            not_found_message: format!("bevy app param: {app_param_ident} not found"),
        },
    ) {
        return err.to_compile_error().into();
    }

    let auto_plugin = if let Some(self_arg) = self_arg {
        quote! {
            <Self as ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin>::build(#self_arg, #app_param_ident);
        }
    } else {
        quote! {
            ::bevy_auto_plugin_shared::modes::global::__internal::AutoPlugin::build(#app_param_ident);
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut output = quote! {
        #(#attrs)*
        #vis #sig
        {
            #auto_plugin
            #block
        }
    };
    output.into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_type(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_type_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_register_type,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_event(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_add_event_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_add_event,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_resource_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_init_resource,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_insert_resource(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalInsertResourceAttributeArgs>(
        attr,
        input,
        "_global_plugin_insert_resource_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_insert_resource,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_init_state(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_state_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_init_state,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_name(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_name_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_auto_name,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_register_state_type(
    attr: CompilerStream,
    input: CompilerStream,
) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_struct_or_enum;
    inner::global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_state_type_",
        require_struct_or_enum,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_register_state_type,
    )
    .into()
}

#[proc_macro_attribute]
#[cfg(feature = "mode_global")]
pub fn global_auto_add_system(attr: CompilerStream, input: CompilerStream) -> CompilerStream {
    use bevy_auto_plugin_shared::util::require_fn;
    inner::global_attribute_outer::<GlobalAddSystemArgs>(
        attr,
        input,
        "_global_plugin_add_system_",
        require_fn,
        bevy_auto_plugin_shared::bevy_app_code_gen::generate_add_system,
    )
    .into()
}
