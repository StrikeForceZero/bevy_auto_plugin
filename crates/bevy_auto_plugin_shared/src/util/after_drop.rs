#![allow(dead_code)]
/** mock up usages

use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

pub struct EmitGuard<'a> {
    out: &'a mut TokenStream,
    original: TokenStream,
    checkpoint: TokenStream,
    done: bool,
}

impl<'a> EmitGuard<'a> {
    pub fn new(out: &'a mut TokenStream) -> Self {
        let original = out.clone(); // cheap, Arc-backed
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
    pub fn checkpoint_item(&mut self, item: &Item) {
        self.checkpoint = quote!(#item);
    }

    /// Save a checkpoint from raw tokens.
    pub fn checkpoint_tokens(&mut self, ts: TokenStream) {
        self.checkpoint = ts;
    }

    /// Finish successfully with a final AST.
    pub fn succeed(mut self, item: Item) {
        *self.out = quote!(#item);
        self.done = true;
    }

    /// Finish with an error, emitting compile_error! + last checkpoint.
    pub fn fail(mut self, err: syn::Error) {
        let ce = err.to_compile_error();
        let ckpt = std::mem::take(&mut self.checkpoint);
        *self.out = quote!(#ce #ckpt);
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
*/
/** usage

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
use std::ops::{Deref, DerefMut};

pub struct AfterDrop<'a, T, F: FnOnce(&'a mut T)> {
    target: Option<&'a mut T>,
    on_drop: Option<F>,
}

impl<'a, T, F: FnOnce(&'a mut T)> AfterDrop<'a, T, F> {
    pub fn new(target: &'a mut T, on_drop: F) -> Self {
        Self {
            target: Some(target),
            on_drop: Some(on_drop),
        }
    }
}

impl<'a, T, F: FnOnce(&'a mut T)> Deref for AfterDrop<'a, T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        self.target.as_deref().unwrap()
    }
}
impl<'a, T, F: FnOnce(&'a mut T)> DerefMut for AfterDrop<'a, T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.target.as_deref_mut().unwrap()
    }
}

impl<'a, T, F: FnOnce(&'a mut T)> Drop for AfterDrop<'a, T, F> {
    fn drop(&mut self) {
        if let (Some(t), Some(f)) = (self.target.take(), self.on_drop.take()) {
            f(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    #[xtest]
    fn test_after_drop() {
        let mut x = 0;
        let mut y = AfterDrop::new(&mut x, |x| *x = 1);
        *y = 2;
        drop(y);
        assert_eq!(x, 1);
    }
}
