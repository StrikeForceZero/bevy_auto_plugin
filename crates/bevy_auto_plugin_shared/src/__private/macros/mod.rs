#[macro_export]
#[doc(hidden)]
macro_rules! ok_or_return_compiler_error_with_span_and_message {
    // Case: Only expression
    ($expr:expr) => {{
        let expr = $expr;
        let span = $crate::syntax::diagnostic::span::get_or_create_span_from_result(&expr);
        $crate::ok_or_return_compiler_error_with_span_and_message!(@internal span, "failed to parse", expr)
    }};

    // Case: Expression, message ident
    ($expr:expr, $message:ident) => {{
        let expr = $expr;
        let span = $crate::syntax::diagnostic::span::get_or_create_span_from_result(&expr);
        $crate::ok_or_return_compiler_error_with_span_and_message!(@internal span, $message, expr)
    }};

    // Case: Span, Expression
    ($span:expr, $expr:expr) => {{
        $crate::ok_or_return_compiler_error_with_span_and_message!(@internal $span, "failed to parse", $expr)
    }};

    // Case: Expression, message
    ($expr:expr, $message:literal) => {{
        let expr = $expr;
        let span = $crate::syntax::diagnostic::span::get_or_create_span_from_result(&expr);
        $crate::ok_or_return_compiler_error_with_span_and_message!(@internal span, $message, expr)
    }};

    // Case: Span, message, Expression
    ($span:expr, $message:literal, $expr:expr) => {{
        $crate::ok_or_return_compiler_error_with_span_and_message!(@internal $span, $message, $expr)
    }};

    // Internal handler (common logic)
    (@internal $span:expr, $message:expr, $expr:expr) => {{
        let span = $span;
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(span, format!("{message}: {e}"))
                    .to_compile_error()
                    .into();
            }
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! ok_or_return_compiler_error {
    // Case: expression, message
    ($expr:expr, $message:expr) => {{
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(e.span(), format!("{message}: {e}"))
                    .to_compile_error()
                    .into();
            }
        }
    }};
    // Case: Only expression
    ($expr:expr) => {{
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return e.to_compile_error().into();
            }
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! parse_macro_input2 {
    ($ts:ident as $ty:ty) => {{
        match syn::parse2::<$ty>($ts) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error(),
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! as_cargo_alias {
    ($name:ident) => {
        ::syn::Ident::new(&$name, ::proc_macro2::Span::call_site())
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bevy_crate_path {
    ($target_crate:ident) => {{
        use ::proc_macro_crate::{FoundCrate, crate_name};
        use ::quote::quote;
        use ::syn::parse2;
        use ::std::{concat, stringify};
        use ::syn::Path;
        use $crate::as_cargo_alias;
        #[allow(clippy::result_large_err)]
        let res: Result::<Path, String> = match crate_name(concat!("bevy_", stringify!($target_crate))) {
            Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy_$target_crate)).unwrap()),
            Ok(FoundCrate::Name(alias)) => {
                let alias_ident = as_cargo_alias!(alias);
                Ok(parse2::<Path>(quote!(::#alias_ident)).unwrap())
            }
            #[allow(unused_variables)]
            Err(err_a) => {
                // fall back to bevy’s re-export
                match crate_name("bevy") {
                    Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy::$target_crate)).unwrap()),
                    Ok(FoundCrate::Name(alias)) => {
                        let alias_ident = as_cargo_alias!(alias);
                        Ok(parse2::<Path>(quote!(::#alias_ident::$target_crate)).unwrap())
                    }
                    #[allow(unused_variables)]
                    Err(err_b) => {
                        #[cfg(feature = "_web")] {
                            // WASM/tests: env not available — use standard path
                            Ok(parse2::<Path>(quote!(::bevy::$target_crate)).unwrap())
                        }
                        #[cfg(not(feature = "_web"))] {
                            let label_a = concat!("bevy_", stringify!($target_crate));
                            let label_b = concat!("bevy::", stringify!($target_crate));
                            Err(format!("\n{label_a}: {err_a}\n{label_b}: {err_b}"))
                        }
                    },
                }
            }
        };
        res
    }};
}

#[cfg(test)]
mod tests {
    use internal_test_proc_macro::xtest;
    use quote::ToTokens;

    #[xtest]
    fn test_bevy_crate_path() {
        assert_eq!(
            bevy_crate_path!(reflect).map(|c| c.to_token_stream().to_string()),
            Ok(":: bevy_reflect".to_string())
        )
    }
}
