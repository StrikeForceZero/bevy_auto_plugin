#[macro_export]
#[doc(hidden)]
macro_rules! ok_or_return_compiler_error {
    // Case 1: Only expression
    ($expr:expr) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), "failed to parse", $expr)
    };

    // Case 2: Span, Expression
    ($span:expr, $expr:expr) => {
        ok_or_return_compiler_error!(@internal $span, "failed to parse", $expr)
    };

    // Case 3: Expression, message
    ($expr:expr, $message:literal) => {
        ok_or_return_compiler_error!(@internal ::proc_macro2::Span::call_site(), $message, $expr)
    };

    // Case 4: Span, message, Expression
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
