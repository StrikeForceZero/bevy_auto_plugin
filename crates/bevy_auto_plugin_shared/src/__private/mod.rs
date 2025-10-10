pub mod attribute;
pub mod auto_plugin_registry;
pub mod expand;
mod macros;

pub(crate) mod paths {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod ecs {
        pub fn resolve() -> Result<syn::Path, String> {
            crate::bevy_crate_path!(ecs)
        }
        pub fn ecs_root_path() -> syn::Path {
            resolve().expect("failed to resolve `bevy_ecs` or `bevy::ecs` - do you have `bevy_ecs` in your dependencies?")
        }
    }

    pub mod reflect {
        use super::*;

        pub fn resolve() -> Result<syn::Path, String> {
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

        pub fn resolve() -> Result<syn::Path, String> {
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
}

#[cfg(test)]
mod tests {
    use crate::bevy_crate_path;
    use internal_test_proc_macro::xtest;
    use quote::ToTokens;

    #[allow(dead_code)]
    fn try_assert_tokens_match(
        plugin: impl std::fmt::Debug,
        input: impl ToString,
        args: impl quote::ToTokens,
    ) -> darling::Result<()> {
        let input = input.to_string();
        if args.to_token_stream().to_string() != input {
            Err(darling::Error::custom(format!(
                concat!(
                    "failed to expand into expected tokens - args: ",
                    stringify!($args_ident),
                    ", plugin: {:?}\n\texpected: {}\n\t     got: {}"
                ),
                plugin,
                input,
                args.to_token_stream(),
            )))
        } else {
            Ok(())
        }
    }

    fn map_resolve_crate(r: Result<syn::Path, String>) -> Result<String, String> {
        r.map(|p| p.into_token_stream().to_string())
    }

    #[xtest]
    fn test_crate_resolve_bevy_ecs() {
        assert_eq!(
            map_resolve_crate(super::paths::ecs::resolve()),
            Ok(":: bevy_ecs".into())
        );
    }

    #[xtest]
    fn test_crate_resolve_bevy_state() {
        assert_eq!(
            map_resolve_crate(super::paths::state::resolve()),
            Ok(":: bevy_state".into())
        );
    }

    #[xtest]
    fn test_crate_resolve_bevy_reflect() {
        assert_eq!(
            map_resolve_crate(super::paths::reflect::resolve()),
            Ok(":: bevy_reflect".into())
        );
    }

    #[xtest]
    fn test_crate_resolve_non_existent_crate() {
        let res = bevy_crate_path!(foobar);
        match res {
            Ok(_) => panic!("expected error"),
            Err(e) => {
                assert!(
                    e.contains("bevy_foobar: Could not find `bevy_foobar`")
                        && e.contains("bevy::foobar: Could not find `bevy`"),
                    "{e:?}"
                );
            }
        }
    }
}
