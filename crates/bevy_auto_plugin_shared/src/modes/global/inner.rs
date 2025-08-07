use crate::attribute_args::{
    GlobalAddSystemArgs, GlobalAutoPluginDeriveArgs, GlobalAutoPluginFnAttributeArgs,
    GlobalInsertResourceAttributeArgs, GlobalMacroArgs, GlobalStructOrEnumAttributeArgs,
    default_app_ident,
};
use crate::bevy_app_code_gen::*;
use crate::modes::global::__internal::_plugin_entry_block;
use crate::util::item::{require_fn, require_struct_or_enum};
use crate::util::meta::fn_meta::{
    FnParamMutabilityCheckErrMessages, is_fn_param_mutable_reference,
};
use crate::{ok_or_return_compiler_error, parse_macro_input2};
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use syn::{FnArg, Item, ItemFn, parse2};

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

    let item: Item = ok_or_return_compiler_error!(syn::parse2(input));

    let ident = ok_or_return_compiler_error!(require(&item));

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote!( #item #output )
}

pub fn global_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    prefix: &'static str,
    require: fn(&Item) -> syn::Result<&Ident>,
    generate_fn: impl Fn(&Ident, <T as GlobalMacroArgs>::Input) -> syn::Result<MacroStream>,
) -> MacroStream
where
    T: GlobalMacroArgs,
{
    global_attribute_inner(attr, input, require, parse2::<T>, |ident, params, _item| {
        let unique_ident = params.get_unique_ident(prefix, ident);
        let plugin = params.plugin().clone();
        let inputs = params.to_input(ident)?;
        let output = inputs
            .map(|input| {
                let app_ident = default_app_ident();
                let register = generate_fn(&app_ident, input)?;
                let expr: syn::ExprClosure = syn::parse_quote!(|#app_ident| { #register });
                let output = _plugin_entry_block(&unique_ident, &plugin, &expr);
                Ok(output)
            })
            .collect::<syn::Result<MacroStream>>()?;
        assert!(
            !output.is_empty(),
            "No plugin entry points were generated for ident: {ident}"
        );
        Ok(output)
    })
}

pub fn expand_global_auto_plugin(attr: MacroStream, input: MacroStream) -> MacroStream {
    use quote::quote;
    use syn::spanned::Spanned;
    let item = parse_macro_input2!(input as ItemFn);
    let params = match parse2::<GlobalAutoPluginFnAttributeArgs>(attr) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error().into(),
    };
    let vis = &item.vis;
    let attrs = &item.attrs;
    let sig = &item.sig;
    let block = &item.block;
    let inputs = &sig.inputs;
    let self_args = inputs
        .into_iter()
        .flat_map(|input| match input {
            FnArg::Receiver(recv) => Some(Ident::new("self", recv.span())),
            FnArg::Typed(_) => None,
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

    quote! {
        #(#attrs)*
        #vis #sig
        {
            #auto_plugin
            #block
        }
    }
}

pub fn expand_global_derive_global_auto_plugin(input: MacroStream) -> MacroStream {
    use darling::FromDeriveInput;
    use quote::ToTokens;
    use quote::quote;
    use syn::DeriveInput;

    let derive_input = parse_macro_input2!(input as DeriveInput);
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

pub fn global_auto_register_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_type_",
        require_struct_or_enum,
        generate_register_type,
    )
}
pub fn global_auto_add_event_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_add_event_",
        require_struct_or_enum,
        generate_add_event,
    )
}
pub fn global_auto_init_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_resource_",
        require_struct_or_enum,
        generate_init_resource,
    )
}
pub fn global_auto_insert_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalInsertResourceAttributeArgs>(
        attr,
        input,
        "_global_plugin_insert_resource_",
        require_struct_or_enum,
        generate_insert_resource,
    )
}
pub fn global_auto_init_state_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_init_state_",
        require_struct_or_enum,
        generate_init_state,
    )
}
pub fn global_auto_name_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_name_",
        require_struct_or_enum,
        generate_auto_name,
    )
}
pub fn global_auto_register_state_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalStructOrEnumAttributeArgs>(
        attr,
        input,
        "_global_plugin_register_state_type_",
        require_struct_or_enum,
        generate_register_state_type,
    )
}
pub fn global_auto_add_system_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalAddSystemArgs>(
        attr,
        input,
        "_global_plugin_add_system_",
        require_fn,
        generate_add_system,
    )
}
