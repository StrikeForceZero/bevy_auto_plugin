#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Default, Clone)]
pub(crate) struct EmitBuilder {
    tokens: TokenStream,
    // LIFO stack of checkpoints for nested phases
    checkpoints: Vec<TokenStream>,
}

impl EmitBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_checkpoint(cp: TokenStream) -> Self {
        Self { tokens: cp.clone(), checkpoints: vec![cp] }
    }

    #[inline]
    pub(crate) fn extend(&mut self, ts: impl ToTokens) -> &mut Self {
        self.tokens.extend(ts.into_token_stream());
        self
    }

    #[inline]
    pub(crate) fn replace(&mut self, ts: impl ToTokens) -> &mut Self {
        self.tokens = ts.into_token_stream();
        self
    }

    /// Push the current tokens as a checkpoint.
    pub(crate) fn push_checkpoint(&mut self) -> &mut Self {
        self.checkpoints.push(self.tokens.clone());
        self
    }

    /// Restore to the last checkpoint. No-op if empty.
    pub(crate) fn restore(&mut self) -> &mut Self {
        if let Some(cp) = self.checkpoints.last() {
            self.tokens = cp.clone();
        }
        self
    }

    /// Restore to the last checkpoint and **pop** it. No-op if empty.
    pub(crate) fn pop_restore(&mut self) -> &mut Self {
        if let Some(cp) = self.checkpoints.pop() {
            self.tokens = cp;
        }
        self
    }

    /// Discard the last checkpoint (keep current tokens). No-op if empty.
    pub(crate) fn discard_checkpoint(&mut self) -> &mut Self {
        self.checkpoints.pop();
        self
    }

    /// Finish successfully.
    pub(crate) fn into_ok<T, E>(self, value: T) -> EmitResult<T, E> {
        Ok((self.tokens, value))
    }

    /// Finish with an error; emits the **last checkpoint** if any, else current tokens.
    pub(crate) fn into_err<T, E>(self, err: E) -> EmitResult<T, E> {
        let fallback =
            if let Some(cp) = self.checkpoints.last() { cp.clone() } else { self.tokens.clone() };
        Err((fallback, err))
    }

    /// Try a phase with automatic checkpointing.
    pub(crate) fn try_phase<E>(&mut self, f: impl FnOnce(&mut Self) -> Result<(), E>) -> &mut Self {
        match f(self.push_checkpoint()) {
            Ok(()) => self.discard_checkpoint(),
            Err(_) => self.pop_restore(),
        }
    }

    #[inline]
    pub fn ok<T, E>(&self, v: T) -> EmitResult<T, E> {
        Ok((self.tokens.clone(), v))
    }
    #[inline]
    pub fn err<T, E>(&self, e: E) -> EmitResult<T, E> {
        Err((self.tokens.clone(), e))
    }

    /// Same as `try_phase` but returns `EmitResult` for easy `?` usage
    pub(crate) fn try_do<T, E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> EmitResult<T, E> {
        self.push_checkpoint();
        match f(self) {
            Ok(v) => {
                self.discard_checkpoint();
                self.ok(v)
            }
            Err(e) => {
                self.pop_restore();
                self.err(e)
            }
        }
    }

    pub(crate) fn try_unit<E>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), E>,
    ) -> Result<(), (TokenStream, E)> {
        self.try_do(f).strip_ok_tokens()
    }

    pub(crate) fn into_tokens(self) -> TokenStream {
        self.tokens
    }

    #[must_use]
    pub(crate) fn take_tokens(&mut self) -> TokenStream {
        std::mem::take(&mut self.tokens)
    }

    pub(crate) fn tokens(&self) -> &TokenStream {
        &self.tokens
    }

    pub(crate) fn tokens_mut(&mut self) -> &mut TokenStream {
        &mut self.tokens
    }
}

impl std::ops::Deref for EmitBuilder {
    type Target = TokenStream;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}
impl std::ops::DerefMut for EmitBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

impl ToTokens for EmitBuilder {
    fn to_tokens(&self, out: &mut TokenStream) {
        out.extend(self.tokens.to_token_stream());
    }
}

pub type EmitResult<T, E> = Result<(TokenStream, T), (TokenStream, E)>;

pub struct Ctx<T>(pub TokenStream, pub T);

impl<T> Ctx<T> {
    pub fn and_then<U, E>(
        self,
        f: impl FnOnce(&TokenStream, T) -> Result<U, E>,
    ) -> EmitResult<U, E> {
        let Ctx(ts, v) = self;
        match f(&ts, v) {
            Ok(u) => Ok((ts, u)),
            Err(e) => Err((ts, e)),
        }
    }
}

impl Ctx<()> {
    pub fn start(ts: TokenStream) -> Ctx<()> {
        Ctx(ts, ())
    }
}

pub trait WithTs {
    type Output;
    fn with_ts(self, ts: TokenStream) -> Self::Output;
}

impl<T, E> WithTs for Result<T, E> {
    type Output = EmitResult<T, E>;
    #[inline]
    fn with_ts(self, ts: TokenStream) -> Self::Output {
        self.map(|v| (ts.clone(), v)).map_err(|e| (ts.clone(), e))
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

pub trait EmitResultExt<T, E> {
    fn split(self) -> (TokenStream, Result<T, E>);
    fn join((ts, res): (TokenStream, Result<T, E>)) -> EmitResult<T, E> {
        res.with_ts(ts)
    }
    fn into_tokens(self) -> TokenStream;
    fn tokens(&self) -> &TokenStream;
    fn map_inner<U>(self, f: impl FnOnce(T) -> U) -> EmitResult<U, E>;
    fn map_inner_err<U>(self, f: impl FnOnce(E) -> U) -> EmitResult<T, U>;
    fn strip_err_tokens(self) -> Result<(TokenStream, T), E>;
    fn strip_ok_tokens(self) -> Result<T, (TokenStream, E)>;
}

impl<T, E> EmitResultExt<T, E> for EmitResult<T, E> {
    #[inline]
    fn split(self) -> (TokenStream, Result<T, E>) {
        match self {
            Ok((ts, v)) => (ts, Ok(v)),
            Err((ts, e)) => (ts, Err(e)),
        }
    }

    #[inline]
    fn into_tokens(self) -> TokenStream {
        match self {
            Ok((ts, _)) => ts,
            Err((ts, _)) => ts,
        }
    }

    #[inline]
    fn tokens(&self) -> &TokenStream {
        match self {
            Ok((ts, _)) => ts,
            Err((ts, _)) => ts,
        }
    }

    #[inline]
    fn map_inner<U>(self, f: impl FnOnce(T) -> U) -> EmitResult<U, E> {
        self.and_then(|(ts, v)| Ok(f(v)).with_ts(ts))
    }

    #[inline]
    fn map_inner_err<U>(self, f: impl FnOnce(E) -> U) -> EmitResult<T, U> {
        self.or_else(|(ts, e)| Err(f(e)).with_ts(ts))
    }

    #[inline]
    fn strip_err_tokens(self) -> Result<(TokenStream, T), E> {
        self.map_err(|(_, e)| e)
    }

    #[inline]
    fn strip_ok_tokens(self) -> Result<T, (TokenStream, E)> {
        self.map(|(_, v)| v)
    }
}

pub type CtxOnly = Ctx<()>;
impl<T, E> From<EmitResult<T, E>> for CtxOnly {
    fn from(value: EmitResult<T, E>) -> Self {
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
    fn builder_extend_and_replace() {
        let mut b = EmitBuilder::new();
        b.extend(quote! { a });
        assert_ts_eq!(&b, quote! { a });

        b.replace(quote! { b });
        assert_ts_eq!(&b, quote! { b });

        b.extend(quote! { c d });
        assert_ts_eq!(&b, quote! { b c d });
    }

    #[xtest]
    fn builder_checkpoint_restore_and_pop() {
        // start with some base
        let mut b = EmitBuilder::from_checkpoint(quote! { base });
        assert_ts_eq!(&b, quote! { base });

        // push + modify + restore (non-pop) should revert to base but leave checkpoint on stack
        b.push_checkpoint().extend(quote! { X }).restore();
        assert_ts_eq!(&b, quote! { base });

        // modify again then pop_restore should also revert to base *and* pop the checkpoint
        b.extend(quote! { TEMP }).pop_restore();
        assert_ts_eq!(&b, quote! { base });

        // push + modify + discard should keep the modified result
        b.push_checkpoint().extend(quote! { KEEP }).discard_checkpoint();
        assert_ts_eq!(&b, quote! { base KEEP });
    }

    #[xtest]
    fn try_phase_ok_keeps_changes() {
        let mut b = EmitBuilder::from_checkpoint(quote! { init });
        b.try_phase(|b| {
            // phase mutates tokens
            b.extend(quote! { ok1 ok2 });
            Ok::<_, syn::Error>(())
        });
        assert_ts_eq!(&b, quote! { init ok1 ok2 });
    }

    #[xtest]
    fn try_phase_err_restores_checkpoint() {
        let mut b = EmitBuilder::from_checkpoint(quote! { start });
        b.try_phase(|b| {
            // mutate then fail; should revert to checkpoint
            b.extend(quote! { broken });
            Err::<(), _>(syn::Error::new(proc_macro2::Span::call_site(), "boom"))
        });
        assert_ts_eq!(&b, quote! { start });
    }

    #[xtest]
    fn into_ok_and_into_err_fallback_rules() {
        // into_ok uses current tokens
        let b1 = {
            let mut b = EmitBuilder::from_checkpoint(quote! { cp });
            b.extend(quote! { work });
            b
        };
        let ok: EmitResult<i32, ()> = b1.into_ok(42);
        match ok {
            Ok((tokens, value)) => {
                assert_eq!(value, 42);
                assert_ts_eq!(&tokens, quote! { cp work });
            }
            _ => panic!("expected Ok"),
        }

        // into_err uses last checkpoint (cp), not the mutated tokens
        let b2 = {
            let mut b = EmitBuilder::from_checkpoint(quote! { cp });
            b.extend(quote! { work });
            b
        };
        let err: EmitResult<(), _> =
            b2.into_err(syn::Error::new(proc_macro2::Span::call_site(), "nope"));
        match err {
            Err((tokens, error)) => {
                let _: syn::Error = error.clone().into();
                assert_ts_eq!(&tokens, quote! { cp });
            }
            _ => panic!("expected Err"),
        }
    }

    #[xtest]
    fn result_split_and_map() {
        // split ok
        {
            let ok: EmitResult<i32, ()> = Ok((quote! { T }, 5));
            let (ts, res) = ok.split();
            assert_ts_eq!(&ts, quote! { T });
            assert_eq!(res.unwrap(), 5);
        }

        // map
        {
            let ok: EmitResult<&'static str, ()> = Ok((quote! { U }, "abc"));
            let mapped = ok.map(|(ts, s)| (ts, s.len()));
            let (ts, res) = mapped.split();
            assert_ts_eq!(ts, quote! { U });
            assert_eq!(res.unwrap(), 3);
        }

        // split err
        {
            let err: EmitResult<(), _> =
                Err((quote! { E }, syn::Error::new(proc_macro2::Span::call_site(), "nope")));
            let (ts, res) = err.split();
            assert_ts_eq!(&ts, quote! { E });
            assert!(res.is_err());
        }
    }

    #[xtest]
    fn to_tokens_impls_emit_builder_and_emit_result() {
        // EmitBuilder → ToTokens
        let mut b = EmitBuilder::new();
        b.extend(quote! { a b c });
        let mut out = TokenStream::new();
        b.to_tokens(&mut out);
        assert_ts_eq!(&out, quote! { a b c });

        // EmitResult Ok → ToTokens
        let ok: EmitResult<(), ()> = Ok((quote! { OKTOK }, ()));
        assert_ts_eq!(ok.tokens(), quote! { OKTOK });

        // EmitResult Err → ToTokens
        let err: EmitResult<(), _> =
            Err((quote! { ERRTOK }, syn::Error::new(proc_macro2::Span::call_site(), "x")));
        assert_ts_eq!(err.into_tokens(), quote! { ERRTOK });
    }
    #[xtest]
    fn restore_noop_when_empty() {
        // If no checkpoints exist, restore/discard/pop_restore are no-ops
        let mut b = EmitBuilder::new();
        b.extend(quote! { X });
        b.restore().discard_checkpoint().pop_restore();
        assert_ts_eq!(b, quote! { X });
    }
}
