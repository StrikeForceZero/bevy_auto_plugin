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
    pub(crate) fn into_ok<T>(self, value: T) -> EmitResult<T> {
        EmitResult::Ok { tokens: self.tokens, value }
    }

    /// Finish with an error; emits the **last checkpoint** if any, else current tokens.
    pub(crate) fn into_err<T, E>(self, err: E) -> EmitResult<T>
    where
        EmitError: From<E>,
    {
        let fallback =
            if let Some(cp) = self.checkpoints.last() { cp.clone() } else { self.tokens.clone() };
        EmitResult::Err { tokens: fallback, error: err.into() }
    }

    /// Try a phase with automatic checkpointing.
    pub(crate) fn try_phase<E>(&mut self, f: impl FnOnce(&mut Self) -> Result<(), E>) -> &mut Self
    where
        EmitError: From<E>,
    {
        match f(self.push_checkpoint()) {
            Ok(()) => self.discard_checkpoint(),
            Err(_e) => self.pop_restore(),
        }
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

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum EmitError {
    #[error("{0}")]
    Syn(#[from] syn::Error),
    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl From<EmitError> for syn::Error {
    fn from(e: EmitError) -> Self {
        match e {
            EmitError::Syn(s) => s,
            EmitError::Darling(d) => d.into(),
        }
    }
}
impl From<EmitError> for darling::Error {
    fn from(e: EmitError) -> Self {
        match e {
            EmitError::Syn(s) => s.into(),
            EmitError::Darling(d) => d,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EmitResult<T> {
    Ok { tokens: TokenStream, value: T },
    Err { tokens: TokenStream, error: EmitError },
}

impl<T> EmitResult<T> {
    pub fn ok(tokens: TokenStream, value: T) -> Self {
        Self::Ok { tokens, value }
    }
    pub fn err(tokens: TokenStream, error: impl Into<EmitError>) -> Self {
        Self::Err { tokens, error: error.into() }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok { .. })
    }
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> EmitResult<U> {
        match self {
            Self::Ok { tokens, value } => EmitResult::Ok { tokens, value: f(value) },
            Self::Err { tokens, error } => EmitResult::Err { tokens, error },
        }
    }
    pub fn map_err(self, f: impl FnOnce(EmitError) -> EmitError) -> Self {
        match self {
            Self::Err { tokens, error } => Self::Err { tokens, error: f(error) },
            ok => ok,
        }
    }

    /// Split into tokens and a plain `Result<T, EmitError>`.
    pub fn split(self) -> (TokenStream, Result<T, EmitError>) {
        match self {
            Self::Ok { tokens, value } => (tokens, Ok(value)),
            Self::Err { tokens, error } => (tokens, Err(error)),
        }
    }
}

impl<T> From<EmitResult<T>> for syn::Result<T> {
    fn from(v: EmitResult<T>) -> Self {
        match v {
            EmitResult::Ok { value, .. } => Ok(value),
            EmitResult::Err { error, .. } => Err(error.into()),
        }
    }
}
impl<T> From<EmitResult<T>> for darling::Result<T> {
    fn from(v: EmitResult<T>) -> Self {
        match v {
            EmitResult::Ok { value, .. } => Ok(value),
            EmitResult::Err { error, .. } => Err(error.into()),
        }
    }
}
impl<T> ToTokens for EmitResult<T> {
    fn to_tokens(&self, out: &mut TokenStream) {
        let ts = match self {
            EmitResult::Ok { tokens, .. } | EmitResult::Err { tokens, .. } => tokens,
        };
        out.extend(ts.clone());
    }
}

#[derive(Debug)]
pub(crate) struct FatalEmit {
    pub tokens: TokenStream,
    pub error: EmitError,
}
impl<T> From<EmitResult<T>> for Result<T, FatalEmit> {
    fn from(v: EmitResult<T>) -> Self {
        match v {
            EmitResult::Ok { value, .. } => Ok(value),
            EmitResult::Err { tokens, error } => Err(FatalEmit { tokens, error }),
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
        let ok: EmitResult<i32> = b1.into_ok(42);
        match ok {
            EmitResult::Ok { tokens, value } => {
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
        let err: EmitResult<()> =
            b2.into_err(syn::Error::new(proc_macro2::Span::call_site(), "nope"));
        match err {
            EmitResult::Err { tokens, error } => {
                let _: syn::Error = error.clone().into(); // convertible
                assert_ts_eq!(&tokens, quote! { cp });
            }
            _ => panic!("expected Err"),
        }
    }

    #[xtest]
    fn result_conversions_to_syn_and_darling() {
        // Ok path -> Ok(_)
        let ok: EmitResult<&'static str> = EmitResult::ok(quote! { t }, "v");
        let syn_ok: syn::Result<_> = ok.clone().into();
        let dar_ok: darling::Result<_> = ok.into();
        assert!(syn_ok.is_ok());
        assert!(dar_ok.is_ok());

        // Err path -> Err(_)
        let e = syn::Error::new(proc_macro2::Span::call_site(), "bad");
        let err: EmitResult<()> = EmitResult::err(quote! { t }, e);
        let syn_err: syn::Result<()> = err.clone().into();
        let dar_err: darling::Result<()> = err.into();
        assert!(syn_err.is_err());
        assert!(dar_err.is_err());
    }

    #[xtest]
    fn result_split_and_map() {
        // split ok
        {
            let ok: EmitResult<i32> = EmitResult::ok(quote! { T }, 5);
            let (ts, res) = ok.split();
            assert_ts_eq!(&ts, quote! { T });
            assert_eq!(res.unwrap(), 5);
        }

        // map
        {
            let ok: EmitResult<&'static str> = EmitResult::ok(quote! { U }, "abc");
            let mapped = ok.map(|s| s.len());
            let (ts, res) = mapped.split();
            assert_ts_eq!(ts, quote! { U });
            assert_eq!(res.unwrap(), 3);
        }

        // split err
        {
            let err: EmitResult<()> = EmitResult::err(
                quote! { E },
                syn::Error::new(proc_macro2::Span::call_site(), "nope"),
            );
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
        let ok: EmitResult<()> = EmitResult::ok(quote! { OKTOK }, ());
        let mut out_ok = TokenStream::new();
        ok.to_tokens(&mut out_ok);
        assert_ts_eq!(&out_ok, quote! { OKTOK });

        // EmitResult Err → ToTokens
        let err: EmitResult<()> = EmitResult::err(
            quote! { ERRTOK },
            syn::Error::new(proc_macro2::Span::call_site(), "x"),
        );
        let mut out_err = TokenStream::new();
        err.to_tokens(&mut out_err);
        assert_ts_eq!(&out_err, quote! { ERRTOK });
    }

    #[xtest]
    fn fatal_emit_conversion() {
        // Ok path -> Ok
        let ok: EmitResult<i32> = EmitResult::ok(quote! { TT }, 9);
        let ok_conv: Result<i32, FatalEmit> = ok.into();
        assert_eq!(ok_conv.unwrap(), 9);

        // Err path -> Err(FatalEmit)
        let err: EmitResult<()> =
            EmitResult::err(quote! { FOO }, darling::Error::unknown_field("bar"));
        let err_conv: Result<(), FatalEmit> = err.into();
        let fe = err_conv.expect_err("should be error");
        assert_ts_eq!(&fe.tokens, quote! { FOO });
        // also check error converts both ways
        let _syn_err: syn::Error = fe.error.clone().into();
        let _dar_err: darling::Error = fe.error.clone().into();
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
