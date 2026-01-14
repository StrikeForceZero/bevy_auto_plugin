#![allow(unused_macros)]

macro_rules! compile_error_with {
    ($err:expr, $user_tokens:expr $(,)?) => {{
        let ce = $err.to_compile_error();
        let tokens = $user_tokens;
        ::quote::quote!( #ce #tokens )
    }};
}

macro_rules! ok_or_emit {
    // Case: expression, tokens, message
    ($expr:expr, $message:expr $(,)?) => {{
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return ::syn::Error::new(e.span(), format!("{message}: {e}")).to_compile_error();
            }
        }
    }};
    // Case: Only expression + tokens
    ($expr:expr) => {{
        match $expr {
            Ok(v) => v,
            Err(e) => return e.to_compile_error(),
        }
    }};
}

macro_rules! ok_or_emit_with {
    // Case: Only expression + tokens
    ($expr:expr, $user_tokens:expr $(,)?) => {{
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let ce = e.to_compile_error();
                let tokens = $user_tokens;
                return ::quote::quote!( #ce #tokens )
            }
        }
    }};
    // Case: expression, tokens, message
    ($expr:expr, $user_tokens:expr, $message:expr $(,)?) => {{
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let ce = ::syn::Error::new(e.span(), format!("{message}: {e}")).to_compile_error();
                let tokens = $user_tokens;
                return ::quote::quote!( #ce #tokens )
            }
        }
    }};
}

macro_rules! parse_macro_input2 {
    ($ts:ident as $ty:ty) => {{
        match syn::parse2::<$ty>($ts) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error(),
        }
    }};
}

macro_rules! parse_macro_input2_or_emit_with {
    ($ts:ident as $ty:ty, $user_tokens:expr $(,)?) => {{
        match ::syn::parse2::<$ty>($ts) {
            Ok(v) => v,
            Err(e) => return $crate::util::macros::compile_error_with!(e, $user_tokens),
        }
    }};
}

macro_rules! as_cargo_alias {
    ($name:expr) => {
        ::syn::Ident::new(&$name, ::proc_macro2::Span::call_site())
    };
}

macro_rules! bevy_crate_path {
    ($target_crate:ident) => {{
        use ::proc_macro_crate::{FoundCrate, crate_name};
        use ::quote::quote;
        use ::syn::parse2;
        use ::std::{concat, stringify};
        use ::syn::Path;
        // unused import for tests
        #[cfg(not(test))]
        use $crate::util::macros::as_cargo_alias;

        #[allow(clippy::result_large_err)]
        let res: Result::<Path, String> = match crate_name("bevy_auto_plugin") {
            Ok(FoundCrate::Itself) => {
                let path_str = concat!("::bevy_auto_plugin::bevy_", stringify!($target_crate));
                Ok(::syn::parse_str(path_str).unwrap())
            }
            Ok(FoundCrate::Name(alias)) => {
                let path_str = format!("::{}::bevy_{}", alias, stringify!($target_crate));
                Ok(::syn::parse_str(&path_str).unwrap())
            }
            Err(_) => {
                // Fallback to searching for the individual bevy crate or bevy itself
                match crate_name(concat!("bevy_", stringify!($target_crate))) {
                    Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy_$target_crate)).unwrap()),
                    Ok(FoundCrate::Name(alias)) => {
                        let alias_ident = as_cargo_alias!(&alias);
                        Ok(parse2::<Path>(quote!(::#alias_ident)).unwrap())
                    }
                    #[allow(unused_variables)]
                    Err(err_a) => {
                        // fall back to bevy’s re-export
                        match crate_name("bevy") {
                            Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy::$target_crate)).unwrap()),
                            Ok(FoundCrate::Name(alias)) => {
                                let alias_ident = as_cargo_alias!(&alias);
                                Ok(parse2::<Path>(quote!(::#alias_ident::$target_crate)).unwrap())
                            }
                            #[allow(unused_variables)]
                            Err(err_b) => {
                                #[cfg(target_arch = "wasm32")] {
                                    // WASM/tests: env not available — use standard path
                                    Ok(parse2::<Path>(quote!(::bevy::$target_crate)).unwrap())
                                }
                                #[cfg(not(target_arch = "wasm32"))] {
                                    let label_a = concat!("bevy_", stringify!($target_crate));
                                    let label_b = concat!("bevy::", stringify!($target_crate));
                                    Err(format!("\n{label_a}: {err_a}\n{label_b}: {err_b}"))
                                }
                            }
                        }
                    }
                }
            }
        };
        res
    }};
}

macro_rules! impl_from_default {
    ($from:ident => ($($to:ident),* $(,)?)) => {
        $(
            impl From<$from> for $to {
                fn from(_: $from) -> Self {
                    Self::default()
                }
            }
        )*
    };
}

#[allow(unused_imports)]
#[rustfmt::skip]
pub(crate) use {
    compile_error_with,
    ok_or_emit,
    ok_or_emit_with,
    parse_macro_input2,
    parse_macro_input2_or_emit_with,
    as_cargo_alias,
    bevy_crate_path,
    impl_from_default,
};

#[cfg(test)]
mod tests {
    use internal_test_proc_macro::xtest;
    use proc_macro2::{
        Span,
        TokenStream,
    };
    use quote::{
        ToTokens,
        quote,
    };

    #[xtest]
    fn test_bevy_crate_path() {
        #[cfg(target_arch = "wasm32")]
        let expected_path = Ok(":: bevy :: reflect".to_string());
        #[cfg(not(target_arch = "wasm32"))]
        let expected_path = Ok(":: bevy_reflect".to_string());
        assert_eq!(
            bevy_crate_path!(reflect).map(|c| c.to_token_stream().to_string()),
            expected_path
        )
    }

    #[test]
    fn test_ok_or_emit_ok() {
        fn process(ts: syn::Result<TokenStream>) -> TokenStream {
            ok_or_emit!(ts)
        }
        assert_eq!(process(Ok(quote! { foo_bar })).to_string(), quote! { foo_bar }.to_string());
    }

    #[test]
    fn test_ok_or_emit_err() {
        fn process(ts: syn::Result<TokenStream>) -> TokenStream {
            ok_or_emit!(ts)
        }
        assert_eq!(
            process(Err(syn::Error::new(Span::call_site(), "error"))).to_string(),
            quote! { :: core :: compile_error ! { "error" } }.to_string()
        );
    }

    #[xtest]
    fn test_ok_or_emit_with_ok() {
        let input = quote! {
            let a = 1;
            let b = 2;
            let c = 3;
        };

        let expected = quote! {
            let foo = 4
        };
        fn process(ts: TokenStream, expected: &TokenStream) -> TokenStream {
            ok_or_emit_with!(syn::Result::Ok(quote! { # expected }), ts)
        }

        assert_eq!(process(input, &expected).to_string(), expected.to_string());
    }

    #[xtest]
    fn test_ok_or_emit_with_err() {
        let input = quote! {
            let a = 1;
            let b = 2;
            let c = 3;
        };
        fn process(ts: TokenStream) -> TokenStream {
            ok_or_emit_with!(Err(syn::Error::new(Span::call_site(), "error")), ts)
        }

        let expected = quote! {
            :: core :: compile_error ! { "error" }
            #input
        };

        assert_eq!(process(input).to_string(), expected.to_string());
    }
}
