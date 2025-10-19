use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Item, UseTree};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum ResolveIdentFromItemError<'a> {
    #[error("Expected function")]
    NotFn(&'a Item),
    #[error("Expected Struct or Enum")]
    NotStructOrEnum(&'a Item),
    #[cfg(feature = "allow_on_use_statements")]
    #[error("Expected Struct or Enum or use statement")]
    NotFnOrUse(&'a Item),
    #[cfg(feature = "allow_on_use_statements")]
    #[error("Expected Struct or Enum or use statement")]
    NotStructOrEnumOrUse(&'a Item),
    #[cfg(feature = "allow_on_use_statements")]
    #[error("unsupported use statement")]
    UnsupportedUseTree(&'a syn::ItemUse),
}

impl ResolveIdentFromItemError<'_> {
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            Self::NotFn(item) => item.span(),
            Self::NotStructOrEnum(item) => item.span(),
            #[cfg(feature = "allow_on_use_statements")]
            Self::NotFnOrUse(item) => item.span(),
            #[cfg(feature = "allow_on_use_statements")]
            Self::NotStructOrEnumOrUse(item) => item.span(),
            #[cfg(feature = "allow_on_use_statements")]
            Self::UnsupportedUseTree(item) => item.span(),
        }
    }
}

impl From<ResolveIdentFromItemError<'_>> for syn::Error {
    fn from(value: ResolveIdentFromItemError) -> Self {
        Self::new(value.span(), value.to_string())
    }
}

pub type IdentFromItemResult<'a> = Result<&'a Ident, ResolveIdentFromItemError<'a>>;

#[cfg(feature = "allow_on_use_statements")]
fn resolve_ident_from_use_item(item: &syn::ItemUse) -> IdentFromItemResult<'_> {
    Ok(match &item.tree {
        UseTree::Path(path) => &path.ident,
        UseTree::Name(name) => &name.ident,
        UseTree::Rename(name) => &name.rename,
        UseTree::Glob(_) => return Err(ResolveIdentFromItemError::UnsupportedUseTree(item)),
        UseTree::Group(_) => return Err(ResolveIdentFromItemError::UnsupportedUseTree(item)),
    })
}

#[cfg(feature = "allow_on_use_statements")]
pub fn resolve_ident_from_fn_or_use_item(item: &Item) -> IdentFromItemResult<'_> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        Item::Use(u) => resolve_ident_from_use_item(u),
        _ => Err(ResolveIdentFromItemError::NotFnOrUse(item)),
    }
}

#[cfg(feature = "allow_on_use_statements")]
pub fn resolve_ident_from_struct_or_enum_or_use_item(item: &Item) -> IdentFromItemResult<'_> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        Item::Use(u) => resolve_ident_from_use_item(u),
        _ => Err(ResolveIdentFromItemError::NotStructOrEnumOrUse(item)),
    }
}

pub fn resolve_ident_from_fn(item: &Item) -> IdentFromItemResult<'_> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        _ => Err(ResolveIdentFromItemError::NotFn(item)),
    }
}

pub fn resolve_ident_from_struct_or_enum(item: &Item) -> IdentFromItemResult<'_> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(ResolveIdentFromItemError::NotStructOrEnum(item)),
    }
}
