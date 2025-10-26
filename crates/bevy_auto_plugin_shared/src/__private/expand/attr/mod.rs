use crate::macro_api::prelude::*;
use crate::util::macros::ok_or_emit_with;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};

pub mod auto_bind_plugin;
pub mod auto_plugin;

fn proc_attribute_outer<T>(attr: MacroStream, input: MacroStream) -> MacroStream
where
    T: ItemAttributeArgs
        + ItemAttributeParse
        + ItemAttributeInput
        + ItemAttributeTarget
        + ItemAttributeUniqueIdent
        + ItemAttributeContext
        + ItemAttributePlugin,
    AppMutationEmitter<T>: ToTokens + EmitAppMutationTokens,
{
    let args = ok_or_emit_with!(
        T::from_attr_input_with_context(attr, input.clone(), Context::default()),
        input
    );
    let mut q = AppMutationEmitter::from_args(args);
    let scrubbed_input = {
        ok_or_emit_with!(q.scrub_item(), q.args.input_item());
        q.args.input_item().to_token_stream()
    };
    let after_item_tokens = ok_or_emit_with!(q.wrap_body(|body| quote! { #body }), scrubbed_input);
    quote! {
        #scrubbed_input
        #after_item_tokens
    }
}

fn proc_attribute_rewrite_outer<T>(attr: MacroStream, input: MacroStream) -> MacroStream
where
    AttrExpansionEmitter<T>: ToTokens,
    T: ItemAttributeArgs + ItemAttributeParse + ItemAttributeInput + ItemAttributeContext,
{
    let args = ok_or_emit_with!(
        T::from_attr_input_with_context(attr, input.clone(), Context::default()),
        input
    );
    AttrExpansionEmitter::from_item_attribute(args).to_token_stream()
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
            proc_attribute_outer::<$args>(attr, input)
        }
    };

    // With parser
    (@one $fn:ident, $args:ty, $parser:expr) => {
        #[inline]
        pub fn $fn(attr: MacroStream, input: MacroStream) -> MacroStream {
            proc_attribute_with_parser_outer::<$args>(attr, input, $parser)
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

gen_auto_attribute_outers! {
    auto_run_on_build          => IaRunOnBuild,
    auto_register_type         => IaRegisterType,
    auto_add_message           => IaAddMessage,
    auto_init_resource         => IaInitResource,
    auto_insert_resource       => IaInsertResource,
    auto_init_state            => IaInitState,
    auto_init_sub_state        => IaInitSubState,
    auto_name                  => IaName,
    auto_register_state_type   => IaRegisterStateType,
    auto_add_system            => IaAddSystem,
    auto_add_observer          => IaAddObserver,
    auto_add_plugin            => IaAddPlugin,
    auto_configure_system_set  => IaConfigureSystemSet,
}

gen_auto_outers! {
    auto_component => IaComponent,
    auto_resource  => IaResource,
    auto_system    => IaSystem,
    auto_event     => IaEvent,
    auto_message   => IaMessage,
    auto_observer  => IaObserver,
    auto_states    => IaState,
}
