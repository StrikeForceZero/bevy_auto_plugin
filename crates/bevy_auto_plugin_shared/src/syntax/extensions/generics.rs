use crate::syntax::extensions::path::PathExt;
use syn::{Lifetime, Path, TraitBound, TraitBoundModifier, TypeParamBound, parse_quote};

/// Injects `Send + Sync + 'static` constraints to any generics that don't have them
pub fn inject_send_sync_static(generics: &mut syn::Generics) {
    let send_marker_path: Path = parse_quote!(::core::marker::Send);
    let sync_marker_path: Path = parse_quote!(::core::marker::Sync);

    let send_marker: TypeParamBound = parse_quote!(#send_marker_path);
    let sync_marker: TypeParamBound = parse_quote!(#sync_marker_path);
    let static_lifetime: TypeParamBound = parse_quote!('static);

    for tp in generics.type_params_mut() {
        // Scan existing bounds so we don't duplicate them.
        let mut has_send = false;
        let mut has_sync = false;
        let mut has_static = false;

        for b in &tp.bounds {
            match b {
                TypeParamBound::Trait(TraitBound {
                    modifier: TraitBoundModifier::None,
                    path,
                    ..
                }) => {
                    if path.is_similar_path_or_ident(&send_marker_path) {
                        has_send = true;
                    }
                    if path.is_similar_path_or_ident(&sync_marker_path) {
                        has_sync = true;
                    }
                }
                TypeParamBound::Trait(TraitBound {
                    modifier: TraitBoundModifier::Maybe(_),
                    ..
                }) => {
                    // e.g. ?Sized — ignore
                }
                TypeParamBound::Lifetime(lt) => {
                    if lt == &Lifetime::new("'static", lt.apostrophe) {
                        has_static = true;
                    }
                }
                _ => {}
            }
        }

        if !has_send {
            tp.bounds.push(send_marker.clone());
        }
        if !has_sync {
            tp.bounds.push(sync_marker.clone());
        }
        if !has_static {
            tp.bounds.push(static_lifetime.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    #[xtest]
    fn inject_send_sync_static_works() {
        let mut generics = parse_quote! {
            <T>
        };
        inject_send_sync_static(&mut generics);
        assert_eq!(
            generics.to_token_stream().to_string(),
            quote! {
                <T: ::core::marker::Send + ::core::marker::Sync + 'static>
            }
            .to_string()
        );
    }

    #[xtest]
    fn inject_missing_works() {
        let mut generics = parse_quote! {
            <T: Send>
        };
        inject_send_sync_static(&mut generics);
        assert_eq!(
            generics.to_token_stream().to_string(),
            quote! {
                <T: Send + ::core::marker::Sync + 'static>
            }
            .to_string()
        );

        let mut generics = parse_quote! {
            <T: Sync>
        };
        inject_send_sync_static(&mut generics);
        assert_eq!(
            generics.to_token_stream().to_string(),
            quote! {
                <T: Sync + ::core::marker::Send + 'static>
            }
            .to_string()
        );

        let mut generics = parse_quote! {
            <T: 'static>
        };
        inject_send_sync_static(&mut generics);
        assert_eq!(
            generics.to_token_stream().to_string(),
            quote! {
                <T: 'static + ::core::marker::Send + ::core::marker::Sync>
            }
            .to_string()
        );
    }

    #[xtest]
    fn inject_replay_works() {
        let mut generics = parse_quote! {
            <T>
        };
        inject_send_sync_static(&mut generics);
        inject_send_sync_static(&mut generics);
        assert_eq!(
            generics.to_token_stream().to_string(),
            quote! {
                <T: ::core::marker::Send + ::core::marker::Sync + 'static>
            }
            .to_string()
        );
    }
}
