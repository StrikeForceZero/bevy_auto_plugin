use crate::macro_api::prelude::*;
use crate::syntax::validated::concrete_path::{
    ConcreteTargetPath, ConcreteTargetPathWithGenericsCollection,
};
use crate::syntax::validated::path_without_generics::{
    PathWithoutGenerics, TryFromPathWithoutGenericsError,
};
use proc_macro2::TokenStream;
use syn::Path;

pub trait ToTokensWithConcreteTargetPath: GenericsArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut TokenStream,
        target: &ConcreteTargetPath,
    );
    fn to_token_stream_with_concrete_target_path(
        &self,
        target: &ConcreteTargetPath,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens_with_concrete_target_path(&mut tokens, target);
        tokens
    }
    fn required_use_statements(&self) -> Vec<syn::ItemUse> {
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct ToTokensIterItem {
    pub required_uses: Vec<syn::ItemUse>,
    pub main_tokens: TokenStream,
}

impl ToTokensIterItem {
    #[cfg(test)]
    pub fn into_main_tokens(self) -> TokenStream {
        self.main_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithTargetPath<T> {
    target: PathWithoutGenerics,
    pub inner: T,
}

impl<T> WithTargetPath<T> {
    pub fn target(&self) -> &PathWithoutGenerics {
        &self.target
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T: GenericsArgs + Clone> WithTargetPath<T> {
    pub fn concrete_target_paths(&self) -> ConcreteTargetPathWithGenericsCollection {
        self.clone().into()
    }
}

impl<T: ToTokensWithConcreteTargetPath + Clone> WithTargetPath<T> {
    /// used to reconstruct `|app| { #(#required_uses)* #main_tokens }`
    pub fn to_tokens_iter_items(&self) -> impl Iterator<Item = ToTokensIterItem> {
        self.concrete_target_paths()
            .into_iter()
            .map(|concrete_target_path| ToTokensIterItem {
                required_uses: self.inner.required_use_statements(),
                main_tokens: self
                    .inner
                    .to_token_stream_with_concrete_target_path(&concrete_target_path),
            })
    }
    #[cfg(test)]
    /// used in tests checking the output of `ToTokensIterItem::main_tokens`
    pub fn to_tokens_iter(&self) -> impl Iterator<Item = TokenStream> {
        self.to_tokens_iter_items()
            .map(ToTokensIterItem::into_main_tokens)
    }
}

impl<T> From<(PathWithoutGenerics, T)> for WithTargetPath<T> {
    fn from(value: (PathWithoutGenerics, T)) -> Self {
        let (target, inner) = value;
        Self { target, inner }
    }
}

impl<T> From<WithTargetPath<T>> for (PathWithoutGenerics, T) {
    fn from(value: WithTargetPath<T>) -> (PathWithoutGenerics, T) {
        (value.target, value.inner)
    }
}

impl<T> TryFrom<(Path, T)> for WithTargetPath<T> {
    type Error = TryFromPathWithoutGenericsError;
    fn try_from(value: (Path, T)) -> Result<Self, Self::Error> {
        let (path, inner) = value;
        let target = PathWithoutGenerics::try_from(path)?;
        Ok(Self { target, inner })
    }
}
