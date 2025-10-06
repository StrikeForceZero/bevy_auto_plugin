pub mod attribute;
pub mod attribute_args;
pub mod context;
mod expr_path_or_call;
mod expr_value;
mod flag_or_list;
pub mod flag_or_meta;
mod generics;
pub mod item_with_attr_match;
mod macros;
pub mod modes;
mod non_empty_path;
mod type_list;
pub mod util;

pub use bevy_app;
pub use bevy_ecs;
pub use bevy_ecs_macros;
pub use bevy_log;
pub use bevy_reflect;
pub use bevy_reflect_derive;
pub use bevy_state;

// module to allow single item globs
pub mod reflect {
    pub mod std_traits {
        pub use bevy_reflect::std_traits::ReflectDefault;
    }
    pub mod component {
        pub use bevy_ecs::reflect::ReflectComponent;
    }
    pub mod resource {
        pub use bevy_ecs::reflect::ReflectResource;
    }
}

pub mod derive {
    pub mod states {
        pub use bevy_state::state::FreelyMutableState;
        pub use bevy_state::state::States;
    }
}

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! parse_attribute_args_with_mode {
        // with meta args
        ($mode:expr, $args_ident:ident, $tokens:expr $(,)?) => {{
            use quote::quote;
            use $crate::__private::attribute_args::attributes::shorthand::Mode;
            use $crate::__private::attribute_args::attributes::shorthand::tokens::ArgsWithMode;
            use $crate::__private::attribute_args::GlobalArgs;
            use $crate::__private::util::extensions::from_meta::FromMetaExt;
            let mode = $mode.clone();
            let macro_path = mode.resolve_macro_path($args_ident::attribute());

            let mut args = vec![];
            if let Mode::Global { plugin } = &mode {
                args.push(quote!(plugin = #plugin));
            }

            if !$tokens.is_empty() {
                args.push($tokens);
            }

            let input = quote! { #[#macro_path( #(#args),* )] };
            let attr: syn::Attribute = syn::parse_quote! { #input };
            let args_with_mode = match &mode {
                Mode::Global { .. } => ArgsWithMode::from(GlobalArgs::<$args_ident>::from_meta(&attr.meta)?),
                _ => ArgsWithMode::new(mode.clone(), $args_ident::from_meta_ext(&attr.meta)?),
            };
            (mode, input, args_with_mode)
        }};

        // path-only form
        ($mode:expr, $args_ident:ident $(,)?) => {{
            use quote::quote;
            use $crate::__private::attribute_args::attributes::shorthand::Mode;
            use $crate::__private::attribute_args::attributes::shorthand::tokens::ArgsWithMode;
            use $crate::__private::attribute_args::GlobalArgs;
            use $crate::__private::util::extensions::from_meta::FromMetaExt;
            let mode = $mode.clone();
            let macro_path = mode.resolve_macro_path($args_ident::attribute());
            let input = quote! { #[#macro_path] };
            let attr: syn::Attribute = syn::parse_quote! { #input };
            let args_with_mode = match &mode {
                Mode::Global { .. } => panic!("global mode requires meta args"),
                _ => ArgsWithMode::new(mode.clone(), $args_ident::from_meta_ext(&attr.meta)?),
            };
            (mode, input, args_with_mode)
        }};
    }

    #[macro_export]
    macro_rules! parse_meta_args {
        ($mode:expr, $args_ident:ident, $( $args:meta ),+ $(,)?) => {{
            $crate::parse_attribute_args_with_mode!($mode, $args_ident, quote! { $( $args ),+ })
        }};

        ($mode:expr, $args_ident:ident $(,)?) => {{
            $crate::parse_attribute_args_with_mode!($mode, $args_ident)
        }};
    }

    #[macro_export]
    macro_rules! parse_vec_args {
        ($mode:expr, $args_ident:ident, $args:expr $(,)?) => {{
            let args = $args;
            $crate::parse_attribute_args_with_mode!($mode, $args_ident, quote! { #(#args),* })
        }};

        ($mode:expr, $args_ident:ident $(,)?) => {{ $crate::parse_attribute_args_with_mode!($mode, $args_ident) }};
    }

    #[macro_export]
    macro_rules! assert_args_expand {
        ($mode:expr, $args_ident:ident, $( $args:meta ),+ $(,)?) => {
            $crate::assert_vec_args_expand!($mode, $args_ident, vec![$( $args ),+])
        };

        ($mode:expr, $args_ident:ident $(,)?) => {
            $crate::assert_vec_args_expand!($mode, $args_ident)
        };
    }

    #[macro_export]
    macro_rules! assert_vec_args_expand {
        ($mode:expr, $args_ident:ident, $args:expr $(,)?) => {{
            let (mode, input, args) = $crate::parse_vec_args!($mode, $args_ident, $args);
            $crate::__private::tests::assert_tokens_match(mode, input, args);
        }};

        ($mode:expr, $args_ident:ident $(,)?) => {{
            let (mode, input, args) = $crate::parse_vec_args!($mode, $args_ident);
            $crate::__private::tests::assert_tokens_match(mode, input, args);
        }};
    }

    #[macro_export]
    macro_rules! try_assert_args_expand {
        ($mode:expr, $args_ident:ident, $args:expr $(,)?) => {{
            let (mode, input, args) = $crate::parse_vec_args!($mode, $args_ident, $args);
            $crate::__private::tests::try_assert_tokens_match(mode, input, args)
        }};

        ($mode:expr, $args_ident:ident $(,)?) => {{
            let (mode, input, args) = $crate::parse_vec_args!($mode, $args_ident);
            $crate::__private::tests::try_assert_tokens_match(mode, input, args)
        }};
    }

    pub fn assert_tokens_match(
        mode: impl std::fmt::Debug,
        input: impl ToString,
        args: impl quote::ToTokens,
    ) {
        let input = input.to_string();
        assert_eq!(
            args.to_token_stream().to_string(),
            input,
            concat!(
                "failed to expand into expected tokens - args: ",
                stringify!($args_ident),
                ", mode: {:?}, args_inner: {}"
            ),
            mode,
            input,
        );
    }

    #[allow(dead_code)]
    pub fn try_assert_tokens_match(
        mode: impl std::fmt::Debug,
        input: impl ToString,
        args: impl quote::ToTokens,
    ) -> darling::Result<()> {
        let input = input.to_string();
        if args.to_token_stream().to_string() != input {
            Err(darling::Error::custom(format!(
                concat!(
                    "failed to expand into expected tokens - args: ",
                    stringify!($args_ident),
                    ", mode: {:?}\n\texpected: {}\n\t     got: {}"
                ),
                mode,
                input,
                args.to_token_stream(),
            )))
        } else {
            Ok(())
        }
    }
}
