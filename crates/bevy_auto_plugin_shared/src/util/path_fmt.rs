use crate::util::extensions::path::PathExt;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Path, PathArguments, PathSegment};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathWithoutGenerics(Path);

#[derive(Error, Debug, Clone)]
pub enum TryFromPathWithoutGenericsError {
    #[error("Path has generics")]
    HasGenerics(Path),
    #[error("Invalid path: {0}")]
    InvalidPath(#[from] syn::Error),
}

impl From<TryFromPathWithoutGenericsError> for syn::Error {
    fn from(value: TryFromPathWithoutGenericsError) -> Self {
        let path = match value {
            TryFromPathWithoutGenericsError::HasGenerics(path) => path,
            TryFromPathWithoutGenericsError::InvalidPath(err) => return err,
        };
        syn::Error::new(
            path.span(),
            TryFromPathWithoutGenericsError::HasGenerics(path).to_string(),
        )
    }
}

impl TryFrom<Path> for PathWithoutGenerics {
    type Error = TryFromPathWithoutGenericsError;
    fn try_from(path: Path) -> Result<Self, Self::Error> {
        if path.has_generics()? {
            return Err(TryFromPathWithoutGenericsError::HasGenerics(path));
        }
        Ok(Self(path))
    }
}

impl From<PathWithoutGenerics> for Path {
    fn from(path: PathWithoutGenerics) -> Self {
        path.0
    }
}

impl<'a> From<&'a PathWithoutGenerics> for &'a Path {
    fn from(path: &'a PathWithoutGenerics) -> Self {
        &path.0
    }
}

impl ToTokens for PathWithoutGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Parse for PathWithoutGenerics {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>()?;
        Ok(Self::try_from(path)?)
    }
}

pub fn path_to_string(path: &Path, strip_spaces: bool) -> String {
    let path_string = quote!(#path).to_string();
    if strip_spaces {
        path_string.replace(" ", "")
    } else {
        path_string
    }
}

pub fn path_to_string_with_spaces(path: &Path) -> String {
    path_to_string(path, false)
}

pub fn ident_to_path_without_generics(ident: Ident) -> PathWithoutGenerics {
    PathWithoutGenerics(Path {
        leading_colon: None,
        segments: {
            let mut segments = Punctuated::new();
            segments.push(PathSegment {
                ident,
                arguments: PathArguments::None,
            });
            segments
        },
    })
}

impl From<Ident> for PathWithoutGenerics {
    fn from(value: Ident) -> Self {
        ident_to_path_without_generics(value)
    }
}

impl From<&Ident> for PathWithoutGenerics {
    fn from(value: &Ident) -> Self {
        ident_to_path_without_generics(value.clone())
    }
}
