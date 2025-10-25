use proc_macro2::Ident;
use syn::Item;
use syn::spanned::Spanned;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum ResolveIdentFromItemError<'a> {
    #[error("Expected function")]
    NotFn(&'a Item),
    #[error("Expected Struct or Enum")]
    NotStructOrEnum(&'a Item),
}

impl ResolveIdentFromItemError<'_> {
    pub fn span(&self) -> proc_macro2::Span {
        match self {
            Self::NotFn(item) => item.span(),
            Self::NotStructOrEnum(item) => item.span(),
        }
    }
}

impl From<ResolveIdentFromItemError<'_>> for syn::Error {
    fn from(value: ResolveIdentFromItemError) -> Self {
        Self::new(value.span(), value.to_string())
    }
}

pub type IdentFromItemResult<'a> = Result<&'a Ident, ResolveIdentFromItemError<'a>>;

pub fn resolve_ident_from_struct_or_enum(item: &Item) -> IdentFromItemResult<'_> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(ResolveIdentFromItemError::NotStructOrEnum(item)),
    }
}
