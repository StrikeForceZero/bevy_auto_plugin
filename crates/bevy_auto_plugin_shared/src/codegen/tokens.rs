use crate::{
    codegen::ExpandAttrs,
    macro_api::prelude::*,
    syntax::validated::non_empty_path::NonEmptyPath,
};
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};
use syn::parse_quote;

pub fn reflect<'a>(idents: impl IntoIterator<Item = &'a Ident>) -> ExpandAttrs {
    let idents = idents.into_iter().collect::<Vec<_>>();
    let use_items = idents
        .iter()
        .copied()
        .filter_map(|ident| {
            Some(match ident.to_string().as_str() {
                // Make the helper available for #[reflect(Component)]
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

pub fn derive_sub_states_path() -> NonEmptyPath {
    let states = crate::__private::paths::state::root_path();
    parse_quote!(#states::state::SubStates)
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
pub fn derive_sub_states<'a>(
    extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
) -> ExpandAttrs {
    ExpandAttrs {
        use_items: vec![crate::__private::paths::state::derive_use_tokens()],
        attrs: vec![derive_from(
            [
                vec![
                    &derive_sub_states_path(),
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

// TODO: we forgot to add this back in, but tests are still passing?
#[allow(dead_code)]
pub fn use_bevy_state_app_ext_states() -> syn::ItemUse {
    let root = crate::__private::paths::state::root_path();
    parse_quote! { use #root::app::AppExtStates as _; }
}

pub fn auto_register_type(args: RegisterTypeAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_register_state_type(args: RegisterStateTypeAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_name(args: NameAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_init_resource(args: InitResourceAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_init_states(args: InitStateAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_init_sub_states(args: InitSubStateAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_add_systems(args: AddSystemAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_add_observer(args: AddObserverAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
pub fn auto_add_message(args: AddMessageAttrEmitter) -> TokenStream {
    args.to_token_stream()
}
