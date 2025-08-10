pub mod attributes;
pub mod derives;
mod schedule_config;

use crate::__private::attribute::AutoPluginItemAttribute;
use crate::__private::generics::GenericsCollection;
use crate::__private::item_with_attr_match::ItemWithAttributeMatch;
use crate::__private::type_list::TypeList;
use crate::__private::util::concrete_path::{
    ConcreteTargetPath, ConcreteTargetPathWithGenericsCollection,
};
use crate::__private::util::path_fmt::{PathWithoutGenerics, TryFromPathWithoutGenericsError};
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream as MacroStream};
use std::hash::Hash;
use syn::parse::Parse;
use syn::{Item, Path};

pub trait GenericsArgs {
    // TODO: see impl ToTokens for Generics
    const TURBOFISH: bool = false;
    fn type_lists(&self) -> &[TypeList];
    fn generics(&self) -> GenericsCollection {
        GenericsCollection(self.type_lists().to_vec())
    }
}

pub trait ToTokensWithConcreteTargetPath: GenericsArgs {
    fn to_tokens_with_concrete_target_path(
        &self,
        tokens: &mut MacroStream,
        target: &ConcreteTargetPath,
    );
    fn to_token_stream_with_concrete_target_path(
        &self,
        target: &ConcreteTargetPath,
    ) -> MacroStream {
        let mut tokens = MacroStream::new();
        self.to_tokens_with_concrete_target_path(&mut tokens, target);
        tokens
    }
}

pub trait ItemAttributeArgs:
    FromMeta + Parse + ToTokensWithConcreteTargetPath + Hash + Clone
{
    fn global_build_prefix() -> &'static str;
    fn attribute() -> AutoPluginItemAttribute;
    fn resolve_item_ident(item: &Item) -> syn::Result<&Ident>;
    fn match_items(items: &[Item]) -> syn::Result<Vec<ItemWithAttributeMatch<Self>>>;
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

    fn _get_unique_ident_string(&self, prefix: &'static str, ident: &Ident) -> String {
        let hash = self._concat_ident_hash(ident);
        format!("{prefix}_{hash}")
    }

    fn get_unique_ident(&self, ident: &Ident) -> Ident {
        let ident_string = self._get_unique_ident_string(Self::Inner::global_build_prefix(), ident);
        Ident::new(&ident_string, ident.span())
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
    pub fn to_tokens_iter(&self) -> impl Iterator<Item = MacroStream> {
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

#[derive(FromMeta, Debug, Clone, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalArgs<T> {
    pub plugin: Path,
    #[darling(flatten)]
    pub inner: T,
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

pub fn default_app_ident() -> Ident {
    Ident::new("app", Span::call_site())
}
