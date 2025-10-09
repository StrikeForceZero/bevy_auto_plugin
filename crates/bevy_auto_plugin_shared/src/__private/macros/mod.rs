#[macro_export]
#[doc(hidden)]
macro_rules! ok_or_return_compiler_error {
    // Case: Only expression
    ($expr:expr) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), "failed to parse", $expr)
    };

    // Case: Expression, message ident
    ($expr:expr, $message:ident) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), $message, $expr)
    };

    // Case: Span, Expression
    ($span:expr, $expr:expr) => {
        ok_or_return_compiler_error!(@internal $span, "failed to parse", $expr)
    };

    // Case: Expression, message
    ($expr:expr, $message:literal) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), $message, $expr)
    };

    // Case: Span, message, Expression
    ($span:expr, $message:literal, $expr:expr) => {
        ok_or_return_compiler_error!(@internal $span, $message, $expr)
    };

    // Internal handler (common logic)
    (@internal $span:expr, $message:expr, $expr:expr) => {{
        let span = $span;
        let message = $message;
        match $expr {
            Ok(v) => v,
            Err(e) => {
                return syn::Error::new(span, format!("{}: {}", message, e))
                    .to_compile_error()
                    .into();
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
        let res: Result::<Path, (::proc_macro_crate::Error, ::proc_macro_crate::Error)> = match crate_name(concat!("bevy_", stringify!($target_crate))) {
            Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy_$target_crate)).unwrap()),
            Ok(FoundCrate::Name(alias)) => {
                let alias_ident = as_cargo_alias!(alias);
                Ok(parse2::<Path>(quote!(::#alias_ident)).unwrap())
            }
            Err(err_a) => {
                // fall back to bevy’s re-export
                match crate_name("bevy") {
                    Ok(FoundCrate::Itself) => Ok(parse2::<Path>(quote!(::bevy::$target_crate)).unwrap()),
                    Ok(FoundCrate::Name(alias)) => {
                        let alias_ident = as_cargo_alias!(alias);
                        Ok(parse2::<Path>(quote!(::#alias_ident::$target_crate)).unwrap())
                    }
                    Err(err_b) => {
                        Err((err_a, err_b))
                    },
                }
            }
        };
        res
    }};
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    #[test]
    fn test_bevy_crate_path() {
        assert_eq!(
            bevy_crate_path!(reflect)
                .map(|c| c.to_token_stream().to_string())
                .map_err(|(a, b)| format!("bevy_*:: {a:?}, bevy::*::{b:?}")),
            Ok(":: bevy_reflect".to_string())
        )
    }
}
