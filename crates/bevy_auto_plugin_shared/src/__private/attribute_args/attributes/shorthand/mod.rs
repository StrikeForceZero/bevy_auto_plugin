use crate::__private::attribute::AutoPluginAttribute;
use crate::__private::non_empty_path::NonEmptyPath;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Path, parse_quote};

pub mod component;
pub mod event;
mod message;
pub mod observer;
pub mod resource;
pub mod states;
pub mod system;

pub mod prelude {
    pub use super::component::ComponentAttributeArgs;
    pub use super::event::EventAttributeArgs;
    pub use super::message::MessageAttributeArgs;
    pub use super::observer::ObserverAttributeArgs;
    pub use super::resource::ResourceAttributeArgs;
    pub use super::states::StatesAttributeArgs;
    pub use super::system::SystemAttributeArgs;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginShortHandAttribute {
    Component,
    Resource,
    Event,
    Message,
    States,
    System,
    Observer,
}

impl AutoPluginShortHandAttribute {
    pub const fn ident_str(&self) -> &'static str {
        match self {
            Self::Component => "auto_component",
            Self::Resource => "auto_resource",
            Self::Event => "auto_event",
            Self::Message => "auto_message",
            Self::States => "auto_states",
            Self::System => "auto_system",
            Self::Observer => "auto_observer",
        }
    }
}

impl AutoPluginAttribute for AutoPluginShortHandAttribute {
    fn ident_str(&self) -> &'static str {
        Self::ident_str(self)
    }
}

pub mod tokens {
    use super::*;
    use crate::__private::attribute_args::AutoPluginAttributeKind;
    use crate::__private::attribute_args::attributes::add_observer::AddObserverAttributeArgs;
    use crate::__private::attribute_args::attributes::prelude::{
        AddMessageAttributeArgs, AddSystemAttributeArgs, AutoNameAttributeArgs,
        InitResourceAttributeArgs, InitStateAttributeArgs, RegisterTypeAttributeArgs,
    };
    use crate::__private::non_empty_path::NonEmptyPath;
    use proc_macro2::Ident;

    #[derive(Debug, Clone)]
    pub struct ArgsWithMode<T: ArgsBackToTokens> {
        pub mode: Mode,
        pub args: T,
    }

    impl<T> ArgsWithMode<T>
    where
        T: ArgsBackToTokens,
    {
        pub fn new(mode: Mode, args: T) -> Self {
            Self { mode, args }
        }
        fn back_to_tokens(&self, tokens: &mut MacroStream) {
            let macro_path = self.args.full_attribute_path(&self.mode);
            let inner_args = self.args.back_to_inner_arg_token_stream();
            let args = if let Mode::Global { plugin } = &self.mode {
                let mut plugin_args = quote! { plugin = #plugin };
                if !inner_args.is_empty() {
                    plugin_args.extend(quote! { , #inner_args });
                }
                plugin_args
            } else {
                inner_args
            };
            tokens.extend(quote! {
                #[#macro_path(#args)]
            });
        }
    }

    // TODO: break this out so theres one for attributes and generic one for just args in general
    pub trait ArgsBackToTokens: AutoPluginAttributeKind {
        fn full_attribute_path(&self, mode: &Mode) -> NonEmptyPath {
            mode.resolve_macro_path(Self::attribute())
        }
        fn back_to_inner_arg_tokens(&self, tokens: &mut MacroStream);
        fn back_to_inner_arg_token_stream(&self) -> MacroStream {
            let mut tokens = MacroStream::new();
            self.back_to_inner_arg_tokens(&mut tokens);
            tokens
        }
    }

    impl<T> ToTokens for ArgsWithMode<T>
    where
        T: ArgsBackToTokens,
    {
        fn to_tokens(&self, tokens: &mut MacroStream) {
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

    pub fn derive_from<'a>(items: impl IntoIterator<Item = &'a NonEmptyPath>) -> MacroStream {
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
    ) -> MacroStream {
        derive_from(
            [
                vec![&derive_component_path()],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )
    }
    pub fn derive_resource<'a>(
        extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
    ) -> MacroStream {
        derive_from(
            [
                vec![&derive_resource_path()],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )
    }
    pub fn derive_event<'a>(
        extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
    ) -> MacroStream {
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
    ) -> MacroStream {
        derive_from(
            [
                vec![&derive_entity_event_path()],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )
    }
    pub fn derive_message<'a>(
        extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
    ) -> MacroStream {
        derive_from(
            [
                vec![&derive_message_path()],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )
    }
    pub fn derive_states<'a>(
        extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
    ) -> ExpandAttrs {
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
    pub fn derive_reflect() -> MacroStream {
        let derive_reflect_path = derive_reflect_path();
        quote! { #[derive(#derive_reflect_path)] }
    }
    pub fn auto_register_type(mode: Mode, args: RegisterTypeAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_name(mode: Mode, args: AutoNameAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_init_resource(mode: Mode, args: InitResourceAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_init_states(mode: Mode, args: InitStateAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_add_systems(mode: Mode, args: AddSystemAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_add_observer(mode: Mode, args: AddObserverAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
    pub fn auto_add_message(mode: Mode, args: AddMessageAttributeArgs) -> MacroStream {
        ArgsWithMode::new(mode, args).to_token_stream()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Global { plugin: Path },
    FlatFile,
    Module,
}

impl Mode {
    pub fn resolve_macro_path<T>(&self, attr: T) -> NonEmptyPath
    where
        T: AutoPluginAttribute,
    {
        let mode = match self {
            Mode::Global { .. } => "global",
            Mode::FlatFile => "flat_file",
            Mode::Module => "module",
        };
        let mode_ident = quote::format_ident!("{}", mode);
        let macro_ident = quote::format_ident!("{}", attr.ident_str());
        parse_quote!(:: bevy_auto_plugin :: modes :: #mode_ident :: prelude :: #macro_ident)
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Global { .. } => "global",
            Mode::FlatFile => "flat_file",
            Mode::Module => "module",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpandAttrs {
    pub attrs: Vec<MacroStream>,
    pub use_items: Vec<MacroStream>,
}

impl PartialEq for ExpandAttrs {
    fn eq(&self, other: &Self) -> bool {
        quote!(#self).to_token_stream().to_string() == quote!(#other).to_token_stream().to_string()
    }
}

impl ExpandAttrs {
    pub fn to_use_attr_ts_tuple(&self) -> (MacroStream, MacroStream) {
        let use_items = &self.use_items;
        let attrs = &self.attrs;
        (
            quote! {
                #(#use_items)*
            },
            quote! {
                #(#attrs)*
            },
        )
    }
    pub fn with(mut self, other: Self) -> Self {
        self.append(other);
        self
    }
    pub fn append(&mut self, other: Self) {
        self.attrs.extend(other.attrs);
        self.use_items.extend(other.use_items);
    }
}

impl ToTokens for ExpandAttrs {
    fn to_tokens(&self, tokens: &mut MacroStream) {
        let use_items = &self.use_items;
        tokens.extend(quote! {
            #(#use_items)*

        });
        let attrs = &self.attrs;
        tokens.extend(quote! {
            #(#attrs)*
        });
    }
}

pub trait ShortHandAttribute {
    fn expand_args(&self, mode: &Mode) -> MacroStream;
    fn expand_attrs(&self, mode: &Mode) -> ExpandAttrs;
}
