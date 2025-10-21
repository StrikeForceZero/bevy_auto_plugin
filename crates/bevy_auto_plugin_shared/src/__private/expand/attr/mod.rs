use crate::__private::attribute::RewriteAttribute;
use crate::__private::auto_plugin_registry::_plugin_entry_block;
use crate::codegen::with_target_path::{ToTokensIterItem, WithTargetPath};
use crate::macro_api::attributes::ItemAttribute;
use crate::macro_api::attributes::ItemAttributeArgs;
use crate::macro_api::attributes::prelude::*;
use crate::macro_api::composed::Composed;
use crate::macro_api::mixins::with_plugin::WithPlugin;
use crate::syntax::diagnostic::kind::item_kind;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use quote::{ToTokens, format_ident, quote};
use std::hash::Hash;
use syn::{Item, parse2};
use thiserror::Error;

pub mod auto_bind_plugin;
pub mod auto_plugin;

fn body<T, G, R>(
    body: impl Fn(MacroStream) -> MacroStream,
) -> impl Fn(&Ident, ItemAttribute<Composed<T, WithPlugin, G>, R>, &Item) -> syn::Result<MacroStream>
where
    T: ItemAttributeArgs + Hash,
{
    move |ident, params, _item| {
        let unique_ident = params.get_unique_ident(ident);
        let plugin = params.plugin().clone();
        let with_target_path = WithTargetPath::from((ident.into(), params));
        let output = with_target_path
            .to_tokens_iter_items()
            .enumerate()
            .map(
                |(
                    ix,
                    ToTokensIterItem {
                        required_uses,
                        main_tokens: tokens,
                    },
                )| {
                    let body = body(tokens);
                    let expr: syn::ExprClosure = syn::parse_quote!(|app| {
                        #(#required_uses)*
                        #body
                    });
                    // required for generics
                    let unique_ident = format_ident!("{unique_ident}_{ix}");
                    let output = _plugin_entry_block(&unique_ident, &plugin, &expr);
                    Ok(output)
                },
            )
            .collect::<syn::Result<MacroStream>>()?;
        assert!(
            !output.is_empty(),
            "No plugin entry points were generated for ident: {ident}"
        );
        Ok(output)
    }
}

pub fn proc_attribute_rewrite_outer<T: RewriteAttribute + FromMeta>(
    attr: MacroStream,
    input: MacroStream,
) -> MacroStream {
    proc_attribute_rewrite_inner::<T>(attr, input).unwrap_or_else(|err| err.to_compile_error())
}

pub fn inject_plugin_arg_for_attributes(attrs: &mut Vec<syn::Attribute>, plugin: &syn::Path) {
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

macro_rules! gen_auto_attribute_outer_call_fns {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                proc_attribute_outer_call_fn::<WithPlugin<$args>>(attr, input)
            }
        )+
    };
}

macro_rules! gen_auto_attribute_outers {
    // Each item:  fn_name => ArgsTy [using <expr>]
    ( $( $fn:ident => $args:ty $(: parser = $parser:expr)? ),+ $(,)? ) => {
        $(
            gen_auto_attribute_outers!(@one $fn, $args $(, $parser)?);
        )+
    };

    // No parser
    (@one $fn:ident, $args:ty) => {
        #[inline]
        pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
            proc_attribute_outer::<WithPlugin<$args>>(attr, input)
        }
    };

    // With parser
    (@one $fn:ident, $args:ty, $parser:expr) => {
        #[inline]
        pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
            proc_attribute_with_parser_outer::<WithPlugin<$args>>(attr, input, $parser)
        }
    };
}

macro_rules! gen_auto_outers {
    ( $( $fn:ident => $args:ty ),+ $(,)? ) => {
        $(
            #[inline]
            pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
                proc_attribute_rewrite_outer::<$args>(attr, input)
            }
        )+
    };
}

gen_auto_attribute_outer_call_fns! {
    auto_run_on_build         => RunOnBuildArgs,
}

gen_auto_attribute_outers! {
    auto_register_type         => RegisterTypeArgs,
    auto_add_message           => AddMessageArgs,
    auto_init_resource         => InitResourceArgs,
    auto_insert_resource       => InsertResourceArgs,
    auto_init_state            => InitStateArgs,
    auto_init_sub_state       => InitSubStateArgs,
    auto_name                  => NameArgs,
    auto_register_state_type   => RegisterStateTypeArgs,
    auto_add_system            => AddSystemArgs,
    auto_add_observer          => AddObserverArgs,
    auto_add_plugin            => AddPluginArgs,
    auto_configure_system_set => ConfigureSystemSetArgs:
        parser = ArgParser::Custom(CustomParser::AttrInput(configure_system_set_args_from_attr_input)),
}

gen_auto_outers! {
    auto_component => ComponentArgs,
    auto_resource  => ResourceArgs,
    auto_system    => SystemArgs,
    auto_event     => EventArgs,
    auto_message   => MessageArgs,
    auto_observer  => ObserverArgs,
    auto_states    => StatesArgs,
}
