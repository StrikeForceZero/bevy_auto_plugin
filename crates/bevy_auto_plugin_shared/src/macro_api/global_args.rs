use crate::codegen::tokens::{ArgsBackToTokens, ArgsWithPlugin};
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::macro_api::attributes::ItemAttributeArgs;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::concrete_path::ConcreteTargetPath;
use crate::syntax::validated::generics::GenericsCollection;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream};
use quote::format_ident;
use std::hash::Hash;
use syn::Path;
use syn::parse::Parse;

pub trait GenericsArgs {
    // TODO: see impl ToTokens for Generics
    const TURBOFISH: bool = false;
    fn type_lists(&self) -> &[TypeList];
    fn generics(&self) -> GenericsCollection {
        GenericsCollection(self.type_lists().to_vec())
    }
}

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalArgs<T> {
    pub plugin: Path,
    #[darling(flatten)]
    pub inner: T,
}

impl<T> GlobalArgs<T> {
    pub fn plugin(&self) -> NonEmptyPath {
        NonEmptyPath::new(self.plugin.clone()).expect("expected plugin to be a valid path")
    }
}

impl<T: ArgsBackToTokens> From<GlobalArgs<T>> for ArgsWithPlugin<T> {
    fn from(value: GlobalArgs<T>) -> Self {
        ArgsWithPlugin::new(value.plugin(), value.inner)
    }
}

impl<T> GenericsArgs for GlobalArgs<T>
where
    T: GenericsArgs,
{
    const TURBOFISH: bool = T::TURBOFISH;
    fn type_lists(&self) -> &[TypeList] {
        self.inner.type_lists()
    }
}

impl<T> ToTokensWithConcreteTargetPath for GlobalArgs<T>
where
    T: ItemAttributeArgs,
{
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut MacroStream,
        target: &ConcreteTargetPath,
    ) {
        self.inner
            .to_tokens_with_concrete_target_path(tokens, target)
    }
}

impl<T> GlobalAttributeArgs for GlobalArgs<T>
where
    T: ItemAttributeArgs,
{
    type Inner = T;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn plugin(&self) -> &Path {
        &self.plugin
    }
}

pub trait GlobalAttributeArgs:
    FromMeta + Parse + ToTokensWithConcreteTargetPath + Hash + Clone
{
    type Inner: ItemAttributeArgs;
    fn inner(&self) -> &Self::Inner;
    fn plugin(&self) -> &Path;

    fn _concat_ident_hash(&self, ident: &Ident) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn _get_unique_ident(&self, prefix: Ident, ident: &Ident) -> Ident {
        let hash = self._concat_ident_hash(ident);
        format_ident!("{prefix}_{hash}")
    }

    fn get_unique_ident(&self, ident: &Ident) -> Ident {
        self._get_unique_ident(Self::Inner::global_build_prefix(), ident)
    }
}
