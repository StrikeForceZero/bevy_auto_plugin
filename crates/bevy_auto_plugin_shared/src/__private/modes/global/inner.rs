use crate::__private::attribute_args::attributes::add_event::AddEventAttributeArgs;
use crate::__private::attribute_args::attributes::add_observer::AddObserverAttributeArgs;
use crate::__private::attribute_args::attributes::add_system::AddSystemAttributeArgs;
use crate::__private::attribute_args::attributes::auto_name::AutoNameAttributeArgs;
use crate::__private::attribute_args::attributes::init_resource::InitResourceAttributeArgs;
use crate::__private::attribute_args::attributes::init_state::InitStateAttributeArgs;
use crate::__private::attribute_args::attributes::insert_resource::InsertResourceAttributeArgs;
use crate::__private::attribute_args::attributes::modes::global::auto_plugin::AutoPluginFnAttributeArgs;
use crate::__private::attribute_args::attributes::modes::resolve_app_param_name;
use crate::__private::attribute_args::attributes::register_state_type::RegisterStateTypeAttributeArgs;
use crate::__private::attribute_args::attributes::register_type::RegisterTypeAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::ShortHandAttribute;
use crate::__private::attribute_args::attributes::shorthand::component::ComponentAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::event::EventAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::observer::ObserverAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::resource::ResourceAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::states::StatesAttributeArgs;
use crate::__private::attribute_args::attributes::shorthand::system::SystemAttributeArgs;
use crate::__private::attribute_args::derives::auto_plugin::GlobalAutoPluginDeriveArgs;
use crate::__private::attribute_args::{
    GlobalArgs, GlobalAttributeArgs, ItemAttributeArgs, WithTargetPath,
};
use crate::__private::modes::global::_plugin_entry_block;
use crate::__private::util::debug::debug_item;
use crate::__private::util::fn_param::require_fn_param_mutable_reference;
use crate::{ok_or_return_compiler_error, parse_macro_input2};
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
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

    let err_msg = format!("Attribute macro is not allowed on {}", debug_item(&item));
    let ident = ok_or_return_compiler_error!(resolve_ident(&item), err_msg);

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
    /// Maps [`crate::__private::util::resolve_ident_from_item::IdentFromItemResult`] to [`syn::Result<&Ident>`]
    fn resolve_item_ident<T: GlobalAttributeArgs>(item: &Item) -> syn::Result<&Ident> {
        T::Inner::resolve_item_ident(item).map_err(|err| syn::Error::new(Span::call_site(), err))
    }

    global_attribute_inner(
        attr,
        input,
        resolve_item_ident::<T>,
        parse2::<T>,
        |ident, params, _item| {
            let unique_ident = params.get_unique_ident(ident);
            let plugin = params.plugin().clone();
            let with_target_path = WithTargetPath::from((ident.into(), params));
            let output = with_target_path
                .to_tokens_iter()
                .map(|input| {
                    let register = quote! { app #input ; };
                    let expr: syn::ExprClosure = syn::parse_quote!(|app| { #register });
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

    // TODO: use helper
    let app_param_ident = match resolve_app_param_name(&item, params.app_param.as_ref()) {
        Ok(ident) => ident,
        Err(err) => return err.into_compile_error(),
    };

    if let Err(err) = require_fn_param_mutable_reference(&item, app_param_ident, "bevy app") {
        return err.to_compile_error();
    }

    let mut impl_plugin = quote! {};

    let auto_plugin_hook = if let Some(self_arg) = self_arg {
        if params.plugin.is_some() {
            return syn::Error::new(
                params.plugin.span(),
                "auto_plugin on trait impl can't specify plugin ident",
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
                "auto_plugin on bare fn can only accept a single parameter with the type &mut bevy::prelude::App",
            )
            .to_compile_error();
        }
        let Some(plugin_ident) = params.plugin else {
            return syn::Error::new(
                params.plugin.span(),
                "auto_plugin on bare fn requires the plugin ident to be specified",
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
                ::std::any::TypeId::of::<Self>()
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

fn global_auto_inner<T: ShortHandAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> syn::Result<MacroStream> {
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::attribute_args::attributes::shorthand::Mode;
    let args = parse2::<GlobalArgs<T>>(attr)?;
    let args_ts = args.inner.expand_attrs(&Mode::Global {
        plugin: args.plugin,
    });
    let input = proc_macro2::TokenStream::from(input);
    Ok(quote! {
        #args_ts
        #input
    })
}

fn global_auto_outer<T: ShortHandAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> MacroStream {
    global_auto_inner::<T>(attr, input).unwrap_or_else(|err| err.to_compile_error())
}

pub fn global_auto_component(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<ComponentAttributeArgs>(attr, input)
}
pub fn global_auto_resource(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<ResourceAttributeArgs>(attr, input)
}
pub fn global_auto_system(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<SystemAttributeArgs>(attr, input)
}
pub fn global_auto_event(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<EventAttributeArgs>(attr, input)
}
pub fn global_auto_observer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<ObserverAttributeArgs>(attr, input)
}
pub fn global_auto_states(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_outer::<StatesAttributeArgs>(attr, input)
}

pub fn global_auto_bind_plugin_inner(
    attr: MacroStream,
    input: MacroStream,
) -> syn::Result<MacroStream> {
    use crate::__private::attribute_args::GlobalArgs;
    use crate::__private::util::extensions::item::ItemAttrsExt;
    use proc_macro2::Span;
    use quote::quote;
    use syn::Item;

    let mut item = parse2::<Item>(input)?;
    let args = parse2::<GlobalArgs<()>>(attr)?;
    let plugin = args.plugin;

    let Ok(mut attrs) = item.take_attrs() else {
        return Err(syn::Error::new(
            Span::call_site(),
            "auto_bind_plugin supports only functions, structs, or enums",
        ));
    };

    inject_plugin_arg_for_attributes(&mut attrs, &plugin);

    let Ok(_) = item.put_attrs(attrs) else {
        unreachable!()
    };

    Ok(quote! { #item })
}

pub fn global_auto_bind_plugin_outer(attr: MacroStream, input: MacroStream) -> MacroStream {
    global_auto_bind_plugin_inner(attr, input).unwrap_or_else(|err| err.to_compile_error())
}

fn inject_plugin_arg_for_attributes(attrs: &mut Vec<syn::Attribute>, plugin: &syn::Path) {
    use syn::Meta;

    for attr in attrs {
        let last = attr
            .path()
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();

        if !last.starts_with("auto_") {
            continue;
        }

        let already_has_plugin = match &attr.meta {
            Meta::List(ml) => list_has_key(ml, "plugin"),
            Meta::Path(_) => false,
            Meta::NameValue(_) => true,
        };

        if already_has_plugin {
            continue;
        }

        inject_plugin_arg(attr, plugin);
    }
}

fn inject_plugin_arg(attr: &mut syn::Attribute, plugin: &syn::Path) {
    use syn::Meta;
    use syn::parse_quote;
    match &attr.meta {
        Meta::Path(path) => *attr = parse_quote!( #[#path(plugin = #plugin)] ),
        Meta::List(ml) => {
            let path = &ml.path;
            let inner = &ml.tokens;
            if inner.is_empty() {
                *attr = parse_quote!( #[#path(plugin = #plugin)] )
            } else {
                *attr = parse_quote!( #[#path(plugin = #plugin, #inner)] )
            }
        }
        _ => {}
    }
}

fn list_has_key(ml: &syn::MetaList, key: &str) -> bool {
    use syn::Meta;
    use syn::Token;
    use syn::parse::Parser;
    use syn::punctuated::Punctuated;
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    match parser.parse2(ml.tokens.clone()) {
        Ok(list) => list.iter().any(|m| match m {
            Meta::NameValue(nv) => nv.path.is_ident(key),
            Meta::List(ml2) => ml2.path.is_ident(key),
            Meta::Path(p) => p.is_ident(key),
        }),
        Err(_) => false,
    }
}
