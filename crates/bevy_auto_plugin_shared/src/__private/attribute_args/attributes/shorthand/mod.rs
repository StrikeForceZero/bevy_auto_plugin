use crate::__private::attribute::AutoPluginAttribute;
use crate::__private::non_empty_path::NonEmptyPath;
use proc_macro2::TokenStream as MacroStream;
use quote::{ToTokens, quote};
use syn::{Path, parse_quote};

pub mod component;
pub mod resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoPluginShortHandAttribute {
    Component,
    Resource,
}

impl AutoPluginShortHandAttribute {
    pub const fn ident_str(&self) -> &'static str {
        match self {
            Self::Component => "auto_component",
            Self::Resource => "auto_resource",
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
    use crate::__private::attribute_args::attributes::prelude::{
        AutoNameAttributeArgs, InitResourceAttributeArgs, RegisterTypeAttributeArgs,
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
                    "Component" => quote! {
                        // Make the helper available for #[reflect(Component)]
                        // TODO: we could eliminate the need for globs if we pass the ident in
                        //  then we can do `ReflectComponent as ReflectComponent$ident`
                        //  #[reflect(Component$ident)]
                        #[allow(unused_imports)]
                        use ::bevy_auto_plugin::__private::shared::__private::reflect::component::*;
                    },
                    "Resource" => quote! {
                        // Make the helper available for #[reflect(Resource)]
                        #[allow(unused_imports)]
                        use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
                    },
                    "Default" => quote! {
                        // Make the helper available for #[reflect(Default)]
                        #[allow(unused_imports)]
                        use ::bevy_auto_plugin::__private::shared::__private::reflect::std_traits::*;
                    },
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

    pub fn reflect_component() -> ExpandAttrs {
        ExpandAttrs {
            attrs: vec![quote! {
                // reflect is helper attribute and expects Ident
                #[reflect(Component)]
            }],
            use_items: vec![quote! {
                // Make the helper available for #[reflect(Component)]
                // TODO: we could eliminate the need for globs if we pass the ident in
                //  then we can do `ReflectComponent as ReflectComponent$ident`
                //  #[reflect(Component$ident)]
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::component::*;
            }],
        }
    }

    pub fn reflect_resource() -> ExpandAttrs {
        ExpandAttrs {
            attrs: vec![quote! {
                // reflect is helper attribute and expects Ident
                #[reflect(Resource)]
            }],
            use_items: vec![quote! {
                // Make the helper available for #[reflect(Resource)]
                // TODO: we could eliminate the need for globs if we pass the ident in
                //  then we can do `ReflectResource as ReflectResource$ident`
                //  #[reflect(Resource$ident)]
                #[allow(unused_imports)]
                use ::bevy_auto_plugin::__private::shared::__private::reflect::resource::*;
            }],
        }
    }

    pub fn derive_from<'a>(items: impl IntoIterator<Item = &'a NonEmptyPath>) -> MacroStream {
        let paths = items.into_iter().collect::<Vec<_>>();
        quote! { #[derive(#(#paths),*)] }
    }

    pub fn derive_component<'a>(
        extra_items: impl IntoIterator<Item = &'a NonEmptyPath>,
    ) -> MacroStream {
        derive_from(
            [
                vec![&parse_quote!(
                    ::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Component
                )],
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
                vec![&parse_quote!(
                    ::bevy_auto_plugin::__private::shared::__private::bevy_ecs_macros::Resource
                )],
                extra_items.into_iter().collect::<Vec<_>>(),
            ]
            .concat(),
        )
    }
    pub fn derive_reflect() -> MacroStream {
        quote! { #[derive(::bevy_auto_plugin::__private::shared::__private::bevy_reflect_derive::Reflect)] }
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
}

#[derive(Debug, Default)]
pub struct ExpandAttrs {
    pub attrs: Vec<MacroStream>,
    pub use_items: Vec<MacroStream>,
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
