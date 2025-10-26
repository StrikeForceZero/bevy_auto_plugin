use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::ops::{Deref, DerefMut};

/** Example usage:
pub fn my_attr(_attr: TokenStream, mut input: TokenStream) -> TokenStream {
    let mut emit = EmitGuard::new(&mut input);

    // Parse from the original snapshot
    let mut item: Item = match syn::parse2(emit.snapshot().clone()) {
        Ok(it) => it,
        Err(e) => return { emit.fail(e); input },
    };

    // Stage 1: scrub helper attributes
    scrub_helpers(&mut item)?;
    emit.checkpoint_item(&item); // helpers removed → safe checkpoint

    // Stage 2: other transforms (can add more checkpoints between steps)
    transform_x(&mut item)?;
    // emit.checkpoint_item(&item);

    // Done
    emit.succeed(item);
    input
}
*/
#[derive(Debug)]
pub struct EmitGuard<'a> {
    out: &'a mut TokenStream,
    original: TokenStream,
    checkpoint: TokenStream,
    done: bool,
}

impl<'a> EmitGuard<'a> {
    pub fn new(out: &'a mut TokenStream) -> Self {
        let original = out.clone();
        Self {
            out,
            checkpoint: original.clone(),
            original,
            done: false,
        }
    }

    /// Read-only view of the original tokens for parsing.
    pub fn snapshot(&self) -> &TokenStream {
        &self.original
    }

    /// Save a checkpoint from an AST state.
    pub fn checkpoint<T: ToTokens>(&mut self, item: impl Into<T>) {
        let item = item.into();
        self.checkpoint = quote!(#item);
    }

    /// Finish successfully with a final AST.
    pub fn succeed<T: ToTokens>(&mut self, item: impl Into<T>) {
        let item = item.into();
        *self.out = quote!(#item);
        self.done = true;
    }

    /// Finish with an error, emitting compile_error! + last checkpoint.
    pub fn fail(&mut self, err: &syn::Error) {
        let ce = err.to_compile_error();
        let checkpoint = std::mem::take(&mut self.checkpoint);
        *self.out = quote!(#ce #checkpoint);
        self.done = true;
    }
}

impl<'a> Drop for EmitGuard<'a> {
    fn drop(&mut self) {
        if !self.done {
            // Safety net: early-returned without succeed/fail,
            // emit the last checkpoint.
            *self.out = self.checkpoint.clone();
        }
    }
}

impl Deref for EmitGuard<'_> {
    type Target = TokenStream;
    fn deref(&self) -> &Self::Target {
        self.out
    }
}
impl DerefMut for EmitGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    #[xtest]
    fn test_emit_guard_early() {
        let mut input = quote!(0);
        let mut emit = EmitGuard::new(&mut input);
        emit.checkpoint::<TokenStream>(quote!(1));
        *emit = quote!(2);
        drop(emit);
        assert_eq!(input.to_string(), quote!(1).to_string());
    }
    #[xtest]
    fn test_emit_guard_success() {
        let mut input = quote!(0);
        let mut emit = EmitGuard::new(&mut input);
        emit.succeed::<TokenStream>(quote!(1));
        drop(emit);
        assert_eq!(input.to_string(), quote!(1).to_string());
    }
}
