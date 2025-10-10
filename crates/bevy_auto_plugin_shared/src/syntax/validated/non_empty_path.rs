use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Meta, Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NonEmptyPath(Path);

impl NonEmptyPath {
    pub fn new(path: Path) -> syn::Result<Self> {
        if path.segments.is_empty() {
            return Err(syn::Error::new(path.span(), "empty path"));
        }
        Ok(Self(path))
    }
    pub fn new_unchecked(path: Path) -> Self {
        Self(path)
    }
    pub fn into_last_ident(self) -> Ident {
        self.0
            .segments
            .into_iter()
            .last()
            .expect("non-empty path")
            .ident
    }
    pub fn last_ident(&self) -> &Ident {
        &self.0.segments.last().expect("non-empty path").ident
    }
    pub fn is_just_ident(&self) -> bool {
        self.0.get_ident().is_some()
    }
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl From<Ident> for NonEmptyPath {
    fn from(value: Ident) -> Self {
        Self(value.into())
    }
}

impl From<&NonEmptyPath> for Ident {
    fn from(value: &NonEmptyPath) -> Self {
        value.last_ident().clone()
    }
}

impl From<NonEmptyPath> for Ident {
    fn from(value: NonEmptyPath) -> Self {
        value.into_last_ident()
    }
}

impl TryFrom<Path> for NonEmptyPath {
    type Error = syn::Error;
    fn try_from(path: Path) -> Result<Self, Self::Error> {
        Self::new(path)
    }
}

impl FromMeta for NonEmptyPath {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        match meta {
            Meta::Path(p) => Ok(NonEmptyPath(p.clone())),
            Meta::List(l) => Err(darling::Error::unsupported_format("list").with_span(l)),
            Meta::NameValue(nv) => {
                Err(darling::Error::unsupported_format("name-value").with_span(nv))
            }
        }
    }
}

impl Parse for NonEmptyPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(NonEmptyPath(input.parse()?))
    }
}

impl ToTokens for NonEmptyPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use quote::quote;
    use syn::punctuated::Punctuated;
    use syn::{PathSegment, Token, parse_quote};

    #[xtest]
    fn test_parse() {
        let input_tokens = quote!(::this);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(input, NonEmptyPath(parse_quote!(#input_tokens)));

        let input_tokens = quote!(Foo);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(input, NonEmptyPath(parse_quote!(#input_tokens)));

        let input_tokens = quote!(this::Foo);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(input, NonEmptyPath(parse_quote!(#input_tokens)));
    }

    #[xtest]
    fn test_to_tokens() {
        let input_tokens = quote!(::this);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
        let input_tokens = quote!(Foo);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
        let input_tokens = quote!(this::Foo);
        let input: NonEmptyPath = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
    }

    #[xtest]
    fn test_empty_path_is_err() {
        assert!(
            NonEmptyPath::new(Path {
                leading_colon: None,
                segments: Punctuated::<PathSegment, Token![::]>::new(),
            })
            .is_err()
        );
    }
}
