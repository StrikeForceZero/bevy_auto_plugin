use crate::codegen::ExpandAttrs;
use crate::macro_api::attributes::AttributeIdent;
use crate::macro_api::attributes::prelude::*;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::parse_quote;

#[derive(Debug, Clone)]
pub struct ArgsWithPlugin<T: ArgsBackToTokens> {
    pub plugin: NonEmptyPath,
    pub args: T,
}

impl<T> ArgsWithPlugin<T>
where
    T: ArgsBackToTokens,
{
    pub fn new(plugin: NonEmptyPath, args: T) -> Self {
        Self { plugin, args }
    }
    fn back_to_tokens(&self, tokens: &mut TokenStream) {
        let macro_path = T::full_attribute_path();
        let inner_args = self.args.back_to_inner_arg_token_stream();
        let args = {
            let plugin = &self.plugin;
            let mut plugin_args = quote! { plugin = #plugin };
            if !inner_args.is_empty() {
                plugin_args.extend(quote! { , #inner_args });
            }
            plugin_args
        };
        tokens.extend(quote! {
            #[#macro_path(#args)]
        });
    }
}

// TODO: break this out so theres one for attributes and generic one for just args in general
pub trait ArgsBackToTokens: AttributeIdent {
    fn back_to_inner_arg_tokens(&self, tokens: &mut TokenStream);
    fn back_to_inner_arg_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.back_to_inner_arg_tokens(&mut tokens);
        tokens
    }
}

impl<T> ToTokens for ArgsWithPlugin<T>
where
    T: ArgsBackToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.back_to_tokens(tokens);
    }
}

pub fn reflect<'a>(idents: impl IntoIterator<Item = &'a Ident>) -> ExpandAttrs {
    let idents = idents.into_iter().collect::<Vec<_>>();
    let use_items = idents
        .iter()
        .copied()
        .filter_map(|ident| {
            Some(match ident.to_string().as_str() {
                // Make the helper available for #[reflect(Component)]
                // TODO: we could eliminate the need for globs if we pass the ident in
                //  then we can do `ReflectComponent as ReflectComponent$ident`
                //  #[reflect(Component$ident)]
                "Component" => crate::__private::paths::reflect::reflect_component_use_tokens(),
                // Make the helper available for #[reflect(Resource)]
                "Resource" => crate::__private::paths::reflect::reflect_resource_use_tokens(),
                // Make the helper available for #[reflect(Default)]
                "Default" => crate::__private::paths::reflect::reflect_default_use_tokens(),
                // Debug, Copy, Clone, PartialEq, Eq, Hash appear to be built in?
                _ => return None,
            })
        })
        .collect::<Vec<_>>();
    ExpandAttrs {
        attrs: vec![quote! {
            // reflect is a helper attribute for Reflect derive and expects Ident
            #[reflect(#(#idents),*)]
        }],
        use_items,
    }
}

pub fn derive_from<'a>(items: impl IntoIterator<Item = &'a NonEmptyPath>) -> TokenStream {
    let paths = items.into_iter().collect::<Vec<_>>();
    quote! { #[derive(#(#paths),*)] }
}

macro_rules! ecs_import {
        ($path:path) => {{
            let root = crate::__private::paths::ecs::ecs_root_path();
            parse_quote!(#root::$path)
        }};
    }

pub fn derive_reflect_path() -> NonEmptyPath {
    let root = crate::__private::paths::reflect::reflect_root_path();
    parse_quote!(#root::Reflect)
}

pub fn derive_component_path() -> NonEmptyPath {
    ecs_import!(prelude::Component)
}

pub fn derive_resource_path() -> NonEmptyPath {
    ecs_import!(prelude::Resource)
}

pub fn derive_event_path() -> NonEmptyPath {
    ecs_import!(prelude::Event)
}

pub fn derive_entity_event_path() -> NonEmptyPath {
    ecs_import!(prelude::EntityEvent)
}

pub fn derive_message_path() -> NonEmptyPath {
    ecs_import!(prelude::Message)
}

pub fn derive_states_path() -> NonEmptyPath {
    let states = crate::__private::paths::state::root_path();
    parse_quote!(#states::state::States)
}

pub fn derive_component<'a>(
    extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
) -> TokenStream {
    derive_from(
        [
            vec![&derive_component_path()],
            extra_items.into_iter().collect::<Vec<_>>(),
        ]
        .concat(),
    )
}
pub fn derive_resource<'a>(extra_items: impl IntoIterator<Item = &'a NonEmptyPath>) -> TokenStream {
    derive_from(
        [
            vec![&derive_resource_path()],
            extra_items.into_iter().collect::<Vec<_>>(),
        ]
        .concat(),
    )
}
pub fn derive_event<'a>(extra_items: impl IntoIterator<Item = &'a NonEmptyPath>) -> TokenStream {
    derive_from(
        [
            vec![&derive_event_path()],
            extra_items.into_iter().collect::<Vec<_>>(),
        ]
        .concat(),
    )
}
pub fn derive_entity_event<'a>(
    extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
) -> TokenStream {
    derive_from(
        [
            vec![&derive_entity_event_path()],
            extra_items.into_iter().collect::<Vec<_>>(),
        ]
        .concat(),
    )
}
pub fn derive_message<'a>(extra_items: impl IntoIterator<Item = &'a NonEmptyPath>) -> TokenStream {
    derive_from(
        [
            vec![&derive_message_path()],
            extra_items.into_iter().collect::<Vec<_>>(),
        ]
        .concat(),
    )
}
pub fn derive_states<'a>(extra_items: impl IntoIterator<Item = &'a NonEmptyPath>) -> ExpandAttrs {
    ExpandAttrs {
        use_items: vec![crate::__private::paths::state::derive_use_tokens()],
        attrs: vec![derive_from(
            [
                vec![
                    &derive_states_path(),
                    &parse_quote!(Debug),
                    &parse_quote!(Default),
                    &parse_quote!(Clone),
                    &parse_quote!(PartialEq),
                    &parse_quote!(Eq),
                    &parse_quote!(Hash),
                ],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )],
    }
}
pub fn derive_reflect() -> TokenStream {
    let derive_reflect_path = derive_reflect_path();
    quote! { #[derive(#derive_reflect_path)] }
}
pub fn auto_register_type(plugin: NonEmptyPath, args: RegisterTypeArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_name(plugin: NonEmptyPath, args: NameArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_init_resource(plugin: NonEmptyPath, args: InitResourceArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_init_states(plugin: NonEmptyPath, args: InitStateArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_add_systems(plugin: NonEmptyPath, args: AddSystemArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_add_observer(plugin: NonEmptyPath, args: AddObserverArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
pub fn auto_add_message(plugin: NonEmptyPath, args: AddMessageArgs) -> TokenStream {
    ArgsWithPlugin::new(plugin, args).to_token_stream()
}
