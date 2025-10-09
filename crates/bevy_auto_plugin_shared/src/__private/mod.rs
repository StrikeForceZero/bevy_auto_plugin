mod any_expr;
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

pub(crate) mod paths {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod ecs {
        pub fn resolve() -> Result<syn::Path, (proc_macro_crate::Error, proc_macro_crate::Error)> {
            crate::bevy_crate_path!(ecs)
        }
        pub fn ecs_root_path() -> syn::Path {
            resolve().expect("failed to resolve `bevy_ecs` or `bevy::ecs` - do you have `bevy_ecs` in your dependencies?")
        }
    }

    pub mod reflect {
        use super::*;

        pub fn resolve() -> Result<syn::Path, (proc_macro_crate::Error, proc_macro_crate::Error)> {
            crate::bevy_crate_path!(reflect)
        }

        pub fn reflect_root_path() -> syn::Path {
            resolve().expect("failed to resolve `bevy_reflect` or `bevy::reflect` - do you have `bevy_reflect` in your dependencies?")
        }

        pub fn reflect_default_use_tokens() -> TokenStream {
            let reflect_root = reflect_root_path();
            quote! {
                #[allow(unused_imports)]
                use #reflect_root::std_traits::ReflectDefault as _;
            }
        }

        pub fn reflect_component_use_tokens() -> TokenStream {
            let ecs_root = ecs::ecs_root_path();
            quote! {
                #[allow(unused_imports)]
                use #ecs_root::reflect::ReflectComponent as _;
            }
        }

        pub fn reflect_resource_use_tokens() -> TokenStream {
            let ecs_root = ecs::ecs_root_path();
            quote! {
                #[allow(unused_imports)]
                use #ecs_root::reflect::ReflectResource as _;
            }
        }
    }

    pub mod state {
        use super::*;

        pub fn resolve() -> Result<syn::Path, (proc_macro_crate::Error, proc_macro_crate::Error)> {
            crate::bevy_crate_path!(state)
        }

        pub fn root_path() -> syn::Path {
            resolve().expect("failed to resolve `bevy_state` or `bevy::state` - do you have `bevy_state` in your dependencies?")
        }

        // breaks parse quote
        pub fn derive_use_tokens() -> TokenStream {
            let state_root = root_path();
            quote! {
                #[allow(unused_imports)]
                use #state_root::state::FreelyMutableState as _;
            }
        }
    }

    pub mod app {
        use super::*;

        pub fn resolve() -> Result<syn::Path, (proc_macro_crate::Error, proc_macro_crate::Error)> {
            crate::bevy_crate_path!(app)
        }

        pub fn root_path() -> syn::Path {
            resolve().expect("failed to resolve `bevy_app` or `bevy::app` - do you have `bevy_app` in your dependencies?")
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

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

    fn map_resolve_crate(
        r: Result<syn::Path, (proc_macro_crate::Error, proc_macro_crate::Error)>,
    ) -> Result<String, String> {
        match r {
            Ok(p) => Ok(p.into_token_stream().to_string()),
            Err((e1, e2)) => Err(format!("{}, {}", e1, e2)),
        }
    }

    #[test]
    pub fn test_crate_resolve_bevy_app() {
        assert_eq!(
            map_resolve_crate(super::paths::app::resolve()),
            Ok(":: bevy_app".into())
        );
    }

    #[test]
    pub fn test_crate_resolve_bevy_ecs() {
        assert_eq!(
            map_resolve_crate(super::paths::ecs::resolve()),
            Ok(":: bevy_ecs".into())
        );
    }

    #[test]
    pub fn test_crate_resolve_bevy_state() {
        assert_eq!(
            map_resolve_crate(super::paths::state::resolve()),
            Ok(":: bevy_state".into())
        );
    }

    #[test]
    pub fn test_crate_resolve_bevy_reflect() {
        assert_eq!(
            map_resolve_crate(super::paths::reflect::resolve()),
            Ok(":: bevy_reflect".into())
        );
    }
}
