use crate::__private::attribute_args::attributes::add_event::AddEventAttributeArgs;
use crate::__private::attribute_args::attributes::add_observer::AddObserverAttributeArgs;
use crate::__private::attribute_args::attributes::add_system::AddSystemAttributeArgs;
use crate::__private::attribute_args::attributes::auto_name::AutoNameAttributeArgs;
use crate::__private::attribute_args::attributes::init_resource::InitResourceAttributeArgs;
use crate::__private::attribute_args::attributes::init_state::InitStateAttributeArgs;
use crate::__private::attribute_args::attributes::insert_resource::InsertResourceAttributeArgs;
use crate::__private::attribute_args::attributes::modes::global::auto_plugin::AutoPluginFnAttributeArgs;
use crate::__private::attribute_args::attributes::register_state_type::RegisterStateTypeAttributeArgs;
use crate::__private::attribute_args::attributes::register_type::RegisterTypeAttributeArgs;
use crate::__private::attribute_args::derives::auto_plugin::GlobalAutoPluginDeriveArgs;
use crate::__private::attribute_args::{
    GlobalArgs, GlobalAttributeArgs, ItemAttributeArgs, WithTargetPath, default_app_ident,
};
use crate::__private::modes::global::_plugin_entry_block;
use crate::__private::util::meta::fn_meta::require_fn_param_mutable_reference;
use crate::{ok_or_return_compiler_error, parse_macro_input2};
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::quote;
use syn::{FnArg, Item, ItemFn, parse2};

fn global_attribute_inner<A, F>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
    resolve_ident: fn(&Item) -> syn::Result<&Ident>,
    parse_attr: fn(MacroStream) -> syn::Result<A>,
    body: F,
) -> MacroStream
where
    A: GlobalAttributeArgs,
    F: FnOnce(&Ident, A, &Item) -> syn::Result<MacroStream>,
{
    let attr = attr.into();
    let input = input.into();

    let item: Item = ok_or_return_compiler_error!(parse2(input));

    let ident = ok_or_return_compiler_error!(resolve_ident(&item));

    let args = ok_or_return_compiler_error!(parse_attr(attr));

    let output = ok_or_return_compiler_error!(body(ident, args, &item));

    quote!( #item #output )
}

pub fn global_attribute_outer<T>(
    attr: impl Into<MacroStream>,
    input: impl Into<MacroStream>,
) -> MacroStream
where
    T: GlobalAttributeArgs,
{
    global_attribute_inner(
        attr,
        input,
        T::Inner::resolve_item_ident,
        parse2::<T>,
        |ident, params, _item| {
            let unique_ident = params.get_unique_ident(ident);
            let plugin = params.plugin().clone();
            let with_target_path = WithTargetPath::from((ident.into(), params));
            let output = with_target_path
                .to_tokens_iter()
                .map(|input| {
                    let app_ident = default_app_ident();
                    let register = quote! { #app_ident #input ; };
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
        },
    )
}

pub fn expand_global_auto_plugin(attr: MacroStream, input: MacroStream) -> MacroStream {
    use quote::quote;
    use syn::spanned::Spanned;
    let item = parse_macro_input2!(input as ItemFn);
    let params = match parse2::<AutoPluginFnAttributeArgs>(attr) {
        Ok(params) => params,
        Err(err) => return err.into_compile_error(),
    };
    let vis = &item.vis;
    let attrs = &item.attrs;
    let sig = &item.sig;
    let fn_ident = &sig.ident;
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

    if let Err(err) = require_fn_param_mutable_reference(&item, app_param_ident, "bevy app") {
        return err.to_compile_error();
    }

    let mut impl_plugin = quote! {};

    let auto_plugin_hook = if let Some(self_arg) = self_arg {
        if params.plugin.is_some() {
            return syn::Error::new(
                params.plugin.span(),
                "global_auto_plugin on trait impl can't specify plugin ident",
            )
            .to_compile_error();
        };
        quote! {
            <Self as ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin>::build(#self_arg, #app_param_ident);
        }
    } else {
        if sig.inputs.len() > 1 {
            return syn::Error::new(
                sig.inputs.span(),
                "global_auto_plugin on bare fn can only accept a single parameter with the type &mut bevy::prelude::App",
            )
            .to_compile_error();
        }
        let Some(plugin_ident) = params.plugin else {
            return syn::Error::new(
                params.plugin.span(),
                "global_auto_plugin on bare fn requires the plugin ident to be specified",
            )
            .to_compile_error();
        };
        impl_plugin.extend(quote! {
            impl ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::Plugin for #plugin_ident {
                fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::App) {
                    #fn_ident(app);
                }
            }
        });
        quote! {
            <#plugin_ident as ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin>::static_build(#app_param_ident);
        }
    };

    quote! {
        #(#attrs)*
        #vis #sig
        {
            #auto_plugin_hook
            #block
        }

        #impl_plugin
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
        Err(err) => return err.write_errors(),
    };
    let ident = &params.ident; // `Test`
    let generics = &params.generics; // `<T1, T2>`
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut output = MacroStream::new();

    output.extend(quote! {
        impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPluginTypeId
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
                Err(err) => return err.into_compile_error(),
            };

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::Plugin for #path_with_generics {
                    fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::App) {
                        <Self as ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin>::build(self, app);
                    }
                }

                impl ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if params.auto_plugin.impl_generic_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::Plugin
                for #ident #ty_generics #where_clause
            {
                fn build(&self, app: &mut ::bevy_auto_plugin::__private::shared::__private::modes::global::bevy_app::App) {
                    <Self as ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin>::build(self, app);
                }
            }
        });
    }

    // TODO: maybe default to this behavior
    if params.auto_plugin.impl_generic_auto_plugin_trait {
        output.extend(quote! {
            impl #impl_generics ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin
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
                Err(err) => return err.into_compile_error(),
            };

            auto_plugin_implemented = true;

            output.extend(quote! {
                impl ::bevy_auto_plugin::__private::shared::__private::modes::global::AutoPlugin for #path_with_generics {}
            });
        }
    }

    if auto_plugin_implemented {
        // satisfy linter #[warn(unused_assignments)]
    }

    output
}

pub fn global_auto_register_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<RegisterTypeAttributeArgs>>(attr, input)
}
pub fn global_auto_add_event_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<AddEventAttributeArgs>>(attr, input)
}
pub fn global_auto_init_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<InitResourceAttributeArgs>>(attr, input)
}
pub fn global_auto_insert_resource_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<InsertResourceAttributeArgs>>(attr, input)
}
pub fn global_auto_init_state_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<InitStateAttributeArgs>>(attr, input)
}
pub fn global_auto_name_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<AutoNameAttributeArgs>>(attr, input)
}
pub fn global_auto_register_state_type_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<RegisterStateTypeAttributeArgs>>(attr, input)
}
pub fn global_auto_add_system_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<AddSystemAttributeArgs>>(attr, input)
}

pub fn global_auto_add_observer_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_attribute_outer::<GlobalArgs<AddObserverAttributeArgs>>(attr, input)
}
