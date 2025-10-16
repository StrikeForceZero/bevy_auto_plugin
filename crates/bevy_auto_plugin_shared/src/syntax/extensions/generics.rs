use syn::{Lifetime, TraitBound, TraitBoundModifier, TypeParamBound, parse_quote};

/// Injects `Send + Sync + 'static` constraints to any generics that don't have them
pub fn inject_send_sync_static(generics: &mut syn::Generics) {
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
                    // TODO: doesn't match `core::marker::Send` or `::core::marker::Send`
                    if path.is_ident("Send") {
                        has_send = true;
                    }
                    // TODO: doesn't match `core::marker::Sync` or `::core::marker::Sync`
                    if path.is_ident("Sync") {
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
            tp.bounds.push(parse_quote!(::core::marker::Send));
        }
        if !has_sync {
            tp.bounds.push(parse_quote!(::core::marker::Sync));
        }
        if !has_static {
            tp.bounds.push(parse_quote!('static));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    #[test]
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

    #[test]
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
}
