#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::ToTokens;

pub type EmitResult<T, V, E> = Result<(T, V), (T, E)>;
pub type EmitOkOnlyResult<T, V, E> = Result<(T, V), E>;
pub type EmitErrOnlyResult<T, V, E> = Result<V, (T, E)>;

pub struct Ctx<T, V>(pub T, pub V);

impl<T, V> Ctx<T, V> {
    pub fn and_then<U, E>(self, f: impl FnOnce(&T, V) -> Result<U, E>) -> EmitResult<T, U, E> {
        let Ctx(ts, v) = self;
        match f(&ts, v) {
            Ok(u) => Ok((ts, u)),
            Err(e) => Err((ts, e)),
        }
    }
    pub fn and_then_mut<U, E>(
        self,
        f: impl FnOnce(&mut T, V) -> Result<U, E>,
    ) -> EmitResult<T, U, E> {
        let Ctx(mut ts, v) = self;
        match f(&mut ts, v) {
            Ok(u) => Ok((ts, u)),
            Err(e) => Err((ts, e)),
        }
    }
}

impl<T> Ctx<T, ()> {
    pub fn start(ts: T) -> Ctx<T, ()> {
        Ctx(ts, ())
    }
}

pub trait WithTs<T> {
    type Output;
    fn with_ts(self, ts: T) -> Self::Output;
}

impl<T, V, E> WithTs<T> for Result<V, E> {
    type Output = EmitResult<T, V, E>;
    #[inline]
    fn with_ts(self, ts: T) -> Self::Output {
        match self {
            Ok(v) => Ok((ts, v)),
            Err(e) => Err((ts, e)),
        }
    }
}

pub trait WithTsError {
    type Output;
    fn with_ts_on_err(self, ts: TokenStream) -> Self::Output;
}

impl<T, E> WithTsError for Result<T, E> {
    type Output = Result<T, (TokenStream, E)>;
    #[inline]
    fn with_ts_on_err(self, ts: TokenStream) -> Self::Output {
        self.map_err(|e| (ts.clone(), e))
    }
}

pub trait EmitResultExt<T, V, E> {
    fn split(self) -> (T, Result<V, E>);
    fn join((ts, res): (T, Result<V, E>)) -> EmitResult<T, V, E> {
        res.with_ts(ts)
    }
    fn into_tokens(self) -> TokenStream
    where
        T: ToTokens;
    fn to_tokens(&self) -> TokenStream
    where
        T: ToTokens;
    fn map_inner<U>(self, f: impl FnOnce(V) -> U) -> EmitResult<T, U, E>;
    fn map_inner_err<U>(self, f: impl FnOnce(E) -> U) -> EmitResult<T, V, U>;
    fn map_err_context<U>(self, f: impl FnOnce(T) -> U) -> Result<(T, V), (U, E)>;
    fn strip_err_context(self) -> EmitOkOnlyResult<T, V, E>;
    fn strip_ok_context(self) -> EmitErrOnlyResult<T, V, E>;
    fn and_then_ctx<U>(self, f: impl FnOnce(&T, V) -> Result<U, E>) -> EmitResult<T, U, E>;
    fn and_then_ctx_mut<U>(self, f: impl FnOnce(&mut T, V) -> Result<U, E>) -> EmitResult<T, U, E>;
}

impl<T, V, E> EmitResultExt<T, V, E> for EmitResult<T, V, E> {
    #[inline]
    fn split(self) -> (T, Result<V, E>) {
        match self {
            Ok((ts, v)) => (ts, Ok(v)),
            Err((ts, e)) => (ts, Err(e)),
        }
    }

    #[inline]
    fn into_tokens(self) -> TokenStream
    where
        T: ToTokens,
    {
        match self {
            Ok((ts, _)) => ts.into_token_stream(),
            Err((ts, _)) => ts.into_token_stream(),
        }
    }

    #[inline]
    fn to_tokens(&self) -> TokenStream
    where
        T: ToTokens,
    {
        match self {
            Ok((ts, _)) => ts.to_token_stream(),
            Err((ts, _)) => ts.to_token_stream(),
        }
    }

    #[inline]
    fn map_inner<U>(self, f: impl FnOnce(V) -> U) -> EmitResult<T, U, E> {
        self.and_then(|(ts, v)| Ok(f(v)).with_ts(ts))
    }

    #[inline]
    fn map_inner_err<U>(self, f: impl FnOnce(E) -> U) -> EmitResult<T, V, U> {
        self.or_else(|(ts, e)| Err(f(e)).with_ts(ts))
    }

    #[inline]
    fn map_err_context<U>(self, f: impl FnOnce(T) -> U) -> Result<(T, V), (U, E)> {
        self.map_err(|(ts, e)| (f(ts), e))
    }

    #[inline]
    fn strip_err_context(self) -> EmitOkOnlyResult<T, V, E> {
        self.map_err(|(_, e)| e)
    }

    #[inline]
    fn strip_ok_context(self) -> EmitErrOnlyResult<T, V, E> {
        self.map(|(_, v)| v)
    }

    #[inline]
    fn and_then_ctx<U>(self, f: impl FnOnce(&T, V) -> Result<U, E>) -> EmitResult<T, U, E> {
        match self {
            Ok((ts, v)) => Ctx(ts, v).and_then(f),
            Err((ts, e)) => Err((ts, e)),
        }
    }

    #[inline]
    fn and_then_ctx_mut<U>(self, f: impl FnOnce(&mut T, V) -> Result<U, E>) -> EmitResult<T, U, E> {
        match self {
            Ok((ts, v)) => Ctx(ts, v).and_then_mut(f),
            Err((ts, e)) => Err((ts, e)),
        }
    }
}

pub type CtxOnly<T> = Ctx<T, ()>;
impl<T, V, E> From<Result<(T, V), (T, E)>> for CtxOnly<T>
where
    T: ToTokens,
{
    fn from(value: Result<(T, V), (T, E)>) -> Self {
        match value {
            Ok((ts, _)) => Ctx(ts, ()),
            Err((ts, _)) => Ctx(ts, ()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use internal_test_util::assert_ts_eq;
    use quote::quote;

    #[xtest]
    fn result_split_and_map() {
        // split ok
        {
            let ok: EmitResult<TokenStream, i32, ()> = Ok((quote! { T }, 5));
            let (ts, res) = ok.split();
            assert_ts_eq!(&ts, quote! { T });
            assert_eq!(res.unwrap(), 5);
        }

        // map
        {
            let ok: EmitResult<TokenStream, &'static str, ()> = Ok((quote! { U }, "abc"));
            let mapped = ok.map(|(ts, s)| (ts, s.len()));
            let (ts, res) = mapped.split();
            assert_ts_eq!(ts, quote! { U });
            assert_eq!(res.unwrap(), 3);
        }

        // split err
        {
            let err: EmitResult<TokenStream, (), _> =
                Err((quote! { E }, syn::Error::new(proc_macro2::Span::call_site(), "nope")));
            let (ts, res) = err.split();
            assert_ts_eq!(&ts, quote! { E });
            assert!(res.is_err());
        }
    }
}
