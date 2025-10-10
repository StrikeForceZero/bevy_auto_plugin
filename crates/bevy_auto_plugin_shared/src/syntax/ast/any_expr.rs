crate::any_expr_enum!(pub AnyExprCallClosureMacroPath: Call, Closure, Macro, Path);
crate::any_expr_enum!(pub AnyExprCallMacroPath: Call, Macro, Path);

#[macro_export]
/// example usage:
/// any_expr_enum!(strict pub AnyExprCallPathClosure: Path, Call, Closure);
/// any_expr_enum!(strict AnyExprCall: Call);
/// any_expr_enum!(AnyExprCallPathClosure: Path, Call, Closure);
/// any_expr_enum!(pub AnyExprCallPathClosure: Path, Call, Closure);
macro_rules! any_expr_enum {

    // permissive version: includes catch-all `Other`
    ($vis:vis $name:ident : $($v:ident),+ $(,)?) => {
        $crate::any_expr_enum!(@impl_permissive $vis $name; $($v),*);
    };
    // strict version: only listed variants
    (strict  $vis:vis $name:ident : $($v:ident),+ $(,)?) => {
        $crate::any_expr_enum!(@impl_strict $vis $name; $($v),*);
    };

    // ---- STRICT IMPL (no catch-all)
    (@impl_strict $vis:vis $name:ident; $($v:ident),+) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $v($crate::any_expr_enum!(@ty $v)),
            )+
        }

        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, ts: &mut ::proc_macro2::TokenStream) {
                match self {
                    $(
                        Self::$v(x) => x.to_tokens(ts),
                    )+
                }
            }
        }

        impl $name {
            fn __from_expr(mut e: ::syn::Expr) -> ::std::result::Result<Self, ::darling::Error> {
                if let ::syn::Expr::Paren(p) = e { e = *p.expr; }
                match e {
                    $( ::syn::Expr::$v(node) => Ok(Self::$v(node)), )+
                    other => Err(::darling::Error::custom(::std::format!(
                        "unsupported expression: expected one of [{}], got `{}`",
                        ::core::stringify!($($v),+), ::quote::quote!(#other)
                    ))),
                }
            }
            fn __debug_tuple_string(&self) -> (&'static str, String) {
                let (ty_string, value_string) = match self {
                    $( Self::$v(x) =>  (::std::stringify!($v), ::quote::ToTokens::to_token_stream(x).to_string()), )+
                };
                (ty_string, value_string)
            }
        }

        $crate::any_expr_enum!(@impls $name);
    };

    // ---- PERMISSIVE IMPL (+any; includes catch-all)
    (@impl_permissive $vis:vis $name:ident; $($v:ident),+) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $vis enum $name {
            $( $v($crate::any_expr_enum!(@ty $v)), )+
            Other(::syn::Expr),
        }

        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, ts: &mut ::proc_macro2::TokenStream) {
                match self {
                    $( Self::$v(x) => x.to_tokens(ts), )+
                    Self::Other(e) => e.to_tokens(ts),
                }
            }
        }

        impl $name {
            fn __from_expr(mut e: ::syn::Expr) -> ::std::result::Result<Self, ::darling::Error> {
                if let ::syn::Expr::Paren(p) = e { e = *p.expr; }
                Ok(match e {
                    $( ::syn::Expr::$v(node) => Self::$v(node), )+
                    other => Self::Other(other),
                })
            }
            fn __debug_tuple_string(&self) -> (&'static str, String) {
                let (ty_string, value_string) = match self {
                    $( Self::$v(x) =>  (::std::stringify!($v), ::quote::ToTokens::to_token_stream(x).to_string()), )+
                    Self::Other(e) => ("Other", ::quote::ToTokens::to_token_stream(e).to_string()),
                };
                (ty_string, value_string)
            }
        }

        $crate::any_expr_enum!(@impls $name);
    };

    // type maps
    (@ty Path)        => { syn::ExprPath };
    (@ty Call)        => { syn::ExprCall };
    (@ty MethodCall)  => { syn::ExprMethodCall };
    (@ty Closure)     => { syn::ExprClosure };
    (@ty Macro)       => { syn::ExprMacro };
    (@ty Lit)         => { syn::ExprLit };
    (@ty Array)       => { syn::ExprArray };
    (@ty Tuple)       => { syn::ExprTuple };
    (@ty Struct)      => { syn::ExprStruct };
    (@ty Paren)       => { syn::ExprParen };
    (@ty Binary)      => { syn::ExprBinary };
    (@ty Unary)       => { syn::ExprUnary };
    (@ty Block)       => { syn::ExprBlock };
    (@ty If)          => { syn::ExprIf };
    (@ty Match)       => { syn::ExprMatch };
    (@ty Field)       => { syn::ExprField };
    (@ty Index)       => { syn::ExprIndex };
    (@ty Range)       => { syn::ExprRange };
    (@ty Repeat)      => { syn::ExprRepeat };
    (@ty Reference)   => { syn::ExprReference };
    (@ty Cast)        => { syn::ExprCast };
    (@ty Let)         => { syn::ExprLet };
    //

    (@impls $name:ident) => {

        impl ::core::convert::From<&$name> for ::proc_macro2::TokenStream {
            fn from(v: &$name) -> Self {
                use ::quote::ToTokens;
                let mut ts = ::proc_macro2::TokenStream::new();
                v.to_tokens(&mut ts);
                ts
            }
        }

        impl ::core::convert::From<$name> for ::proc_macro2::TokenStream {
            fn from(v: $name) -> Self {
                (&v).into()
            }
        }

        impl ::darling::FromMeta for $name {
            fn from_meta(m: &::syn::Meta) -> ::std::result::Result<Self, ::darling::Error> {
                use ::syn::parse::Parser;
                match m {
                    ::syn::Meta::List(list) => {
                        let parser = syn::punctuated::Punctuated::<::syn::Expr, ::syn::Token![,]>::parse_terminated;
                        let elems = parser.parse2(list.tokens.clone()).map_err(::darling::Error::custom)?;
                        let mut it = elems.into_iter();
                        let expr = it.next().ok_or_else(|| darling::Error::custom("expected one expression"))?;
                        if it.next().is_some() {
                            return Err(darling::Error::custom("expected exactly one expression"));
                        }
                        Self::__from_expr(expr)
                    }
                    ::syn::Meta::NameValue(nv) => Self::__from_expr(nv.value.clone()),
                    ::syn::Meta::Path(_) => Err(::darling::Error::custom("expected `attr = <expr>` or `attr(<expr>)`")),
                }
            }
        }

        impl ::syn::parse::Parse for $name {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                let elems = ::syn::punctuated::Punctuated::<::syn::Expr, ::syn::Token![,]>::parse_terminated(input)?;
                let mut it = elems.into_iter();
                let Some(expr) = it.next() else {
                    return Err(::syn::Error::new(input.span(), "expected exactly one expression"));
                };
                if it.next().is_some() {
                    return Err(::syn::Error::new(input.span(), "expected exactly one expression"));
                }
                $name::__from_expr(expr).map_err(|e| syn::Error::new(input.span(), e.to_string()))
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let (ty_string, value_string) = self.__debug_tuple_string();
                let ty_string = ::std::format!("{}::{ty_string}", ::std::stringify!($name));
                f.debug_tuple(&ty_string)
                    .field(&::std::format_args!("{value_string}"))
                    .finish()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use darling::FromMeta;
    use internal_test_proc_macro::xtest;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_quote, parse2};

    fn map_err_to_string<T, E: std::fmt::Display>(r: Result<T, E>) -> Result<T, &'static str> {
        r.map_err(|e| &*format!("{e}").leak())
    }

    fn try_from_args<T: FromMeta>(args: TokenStream) -> Result<T, &'static str> {
        let meta = map_err_to_string(parse2::<syn::Meta>(quote! { foo = #args }))
            .map_err(|e| &*format!("THIS: {e}").leak())?;
        let res = map_err_to_string(T::from_meta(&meta))?;
        Ok(res)
    }

    fn try_from_meta_args<T: FromMeta>(args: TokenStream) -> Result<T, &'static str> {
        let meta = map_err_to_string(parse2::<syn::Meta>(quote! { foo = #args }))
            .map_err(|e| &*format!("THIS: {e}").leak())?;
        let res = map_err_to_string(T::from_meta(&meta))?;
        Ok(res)
    }

    mod strict {
        use super::*;

        #[xtest]
        fn test_parse2_any_expr_single() {
            any_expr_enum!(strict AnyExprTest: Closure);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // call - invalid
            let input = quote!(foo());
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Err("unsupported expression: expected one of [Closure], got `foo ()`")
            );
        }

        #[xtest]
        fn test_from_meta_any_single() {
            any_expr_enum!(strict AnyExprTest: Closure);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // call - invalid
            let input = quote!(foo());
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Err("unsupported expression: expected one of [Closure], got `foo ()`")
            );
        }

        #[xtest]
        fn test_parse2_any_multiple() {
            any_expr_enum!(strict AnyExprTest: Closure, Path);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // path - valid
            let input = quote!(foo);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Path(parse_quote!(#input)))
            );
            // call - invalid
            let input = quote!(foo());
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Err("unsupported expression: expected one of [Closure, Path], got `foo ()`")
            );
        }

        #[xtest]
        fn test_from_meta_any_multiple() {
            any_expr_enum!(strict AnyExprTest: Closure, Path);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // path - valid
            let input = quote!(foo);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Path(parse_quote!(#input)))
            );
            // call - invalid
            let input = quote!(foo());
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Err("unsupported expression: expected one of [Closure, Path], got `foo ()`")
            );
        }
    }

    mod permissive {
        use super::*;

        #[xtest]
        fn test_parse2_any_expr_single() {
            any_expr_enum!(AnyExprTest: Closure);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // call - valid (other)
            let input = quote!(foo());
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Other(parse_quote!(#input)))
            );
        }

        #[xtest]
        fn test_from_meta_any_single() {
            any_expr_enum!(AnyExprTest: Closure);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // call - valid (other)
            let input = quote!(foo());
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Other(parse_quote!(#input)))
            );
        }

        #[xtest]
        fn test_parse2_any_multiple() {
            any_expr_enum!(AnyExprTest: Closure, Path);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // path - valid
            let input = quote!(foo);
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Path(parse_quote!(#input)))
            );
            // call - valid (other)
            let input = quote!(foo());
            assert_eq!(
                try_from_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Other(parse_quote!(#input)))
            );
        }

        #[xtest]
        fn test_from_meta_any_multiple() {
            any_expr_enum!(AnyExprTest: Closure, Path);
            // closure - valid
            let input = quote!(|| 1);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Closure(parse_quote!(#input)))
            );
            // path - valid
            let input = quote!(foo);
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Path(parse_quote!(#input)))
            );
            // call - valid (other)
            let input = quote!(foo());
            assert_eq!(
                try_from_meta_args::<AnyExprTest>(input.clone()),
                Ok(AnyExprTest::Other(parse_quote!(#input)))
            );
        }
    }

    #[xtest]
    fn test_debug() {
        // Closure
        {
            any_expr_enum!(AnyExprTest: Closure);
            let any_expr: AnyExprTest = parse_quote!(|| 1);
            assert_eq!(&format!("{any_expr:?}"), "AnyExprTest::Closure(| | 1)");
        }

        // Path
        {
            any_expr_enum!(AnyExprTest: Path);
            let any_expr: AnyExprTest = parse_quote!(foo);
            assert_eq!(&format!("{any_expr:?}"), "AnyExprTest::Path(foo)");
        }

        // Other
        {
            any_expr_enum!(AnyExprTest: Closure);
            let any_expr: AnyExprTest = parse_quote!(foo);
            assert_eq!(&format!("{any_expr:?}"), "AnyExprTest::Other(foo)");
        }
    }
}
