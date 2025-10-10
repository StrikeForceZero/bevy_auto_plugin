use crate::macro_api::global_args::GenericsArgs;
use crate::util::concrete_path::{ConcreteTargetPath, ConcreteTargetPathWithGenericsCollection};
use crate::util::path_fmt::{PathWithoutGenerics, TryFromPathWithoutGenericsError};
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
    pub fn to_tokens_iter(&self) -> impl Iterator<Item = TokenStream> {
        self.concrete_target_paths()
            .into_iter()
            .map(|concrete_target_path| {
                self.inner
                    .to_token_stream_with_concrete_target_path(&concrete_target_path)
            })
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
