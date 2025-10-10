use crate::__private::attribute::{AutoPluginAttribute, AutoPluginItemAttribute};
use crate::codegen::tokens::{ArgsBackToTokens, ArgsWithPlugin};
use crate::codegen::with_target_path::ToTokensWithConcreteTargetPath;
use crate::syntax::ast::type_list::TypeList;
use crate::syntax::validated::generics::GenericsCollection;
use crate::syntax::validated::non_empty_path::NonEmptyPath;
use crate::util::concrete_path::ConcreteTargetPath;
use crate::util::resolve_ident_from_item::IdentFromItemResult;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream as MacroStream};
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

pub trait AutoPluginAttributeKind {
    type Attribute: AutoPluginAttribute;
    fn attribute() -> Self::Attribute;
}

pub trait ItemAttributeArgs:
    AutoPluginAttributeKind<Attribute = AutoPluginItemAttribute>
    + FromMeta
    + Parse
    + ToTokensWithConcreteTargetPath
    + Hash
    + Clone
{
    fn global_build_prefix() -> &'static str;
    fn resolve_item_ident(item: &Item) -> IdentFromItemResult<'_>;
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
