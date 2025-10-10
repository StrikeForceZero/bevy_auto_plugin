pub mod attribute;
pub mod auto_plugin_registry;
pub mod expand;

macro_rules! bevy_crate_err_message {
    ($ident:ident) => {
        concat!(
            "failed to resolve `bevy_",
            stringify!($ident),
            "` or `bevy::",
            stringify!($ident),
            "` - do you have `bevy_",
            stringify!($ident),
            "` in your dependencies?"
        )
    };
}

pub(crate) mod paths {
    use crate::util::macros::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod ecs {
        use super::*;

        pub fn resolve() -> Result<syn::Path, String> {
            bevy_crate_path!(ecs)
        }
        pub fn ecs_root_path() -> syn::Path {
            resolve().expect(bevy_crate_err_message!(ecs))
        }
    }

    pub mod reflect {
        use super::*;

        pub fn resolve() -> Result<syn::Path, String> {
            bevy_crate_path!(reflect)
        }

        pub fn reflect_root_path() -> syn::Path {
            resolve().expect(bevy_crate_err_message!(reflect))
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
            bevy_crate_path!(state)
        }

        pub fn root_path() -> syn::Path {
            resolve().expect(bevy_crate_err_message!(state))
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
    use crate::util::macros::*;
    use internal_test_proc_macro::xtest;
    use quote::ToTokens;

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
