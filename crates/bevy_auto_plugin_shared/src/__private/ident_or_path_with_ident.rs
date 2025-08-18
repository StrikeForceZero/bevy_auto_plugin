use darling::{FromMeta, Result};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Meta, Path};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct IdentOrPathWithIdent {
    path: Option<Path>,
    ident: Ident,
}

impl From<Ident> for IdentOrPathWithIdent {
    fn from(ident: Ident) -> Self {
        Self { path: None, ident }
    }
}

impl TryFrom<Path> for IdentOrPathWithIdent {
    type Error = darling::Error;

    fn try_from(path: Path) -> std::result::Result<Self, Self::Error> {
        Self::try_from(&path)
    }
}

impl TryFrom<&Path> for IdentOrPathWithIdent {
    type Error = darling::Error;

    fn try_from(path: &Path) -> std::result::Result<Self, Self::Error> {
        if let Some(ident) = path.get_ident() {
            return Ok(IdentOrPathWithIdent {
                path: None,
                ident: ident.clone(),
            });
        }
        let Some(seg) = path.segments.last() else {
            return Err(darling::Error::custom("invalid-path").with_span(path));
        };
        Ok(IdentOrPathWithIdent {
            path: Some(path.clone()),
            ident: seg.ident.clone(),
        })
    }
}

impl FromMeta for IdentOrPathWithIdent {
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[this]`
            Meta::Path(path) => Self::try_from(path),

            // `#[this(A, B)]`
            Meta::List(list) => Err(darling::Error::unsupported_format("list").with_span(list)),

            // Not supported: `#[this = ...]`
            Meta::NameValue(nv) => {
                Err(darling::Error::unsupported_format("name-value").with_span(nv))
            }
        }
    }
}

impl Parse for IdentOrPathWithIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>()?;
        Self::try_from(&path).map_err(|err| syn::Error::new(path.span(), err))
    }
}

impl ToTokens for IdentOrPathWithIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(path) = &self.path {
            path.to_tokens(tokens);
            return;
        }
        self.ident.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_parse() {
        let input_tokens = quote!(::this);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(input.path, Some(parse_quote!(#input_tokens)));
        assert_eq!(input.ident, "this");

        let input_tokens = quote!(Foo);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(input.path, None);
        assert_eq!(input.ident, "Foo");

        let input_tokens = quote!(this::Foo);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(input.path, Some(parse_quote!(#input_tokens)));
        assert_eq!(input.ident, "Foo");
    }

    #[internal_test_proc_macro::xtest]
    fn test_to_tokens() {
        let input_tokens = quote!(::this);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
        let input_tokens = quote!(Foo);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
        let input_tokens = quote!(this::Foo);
        let input: IdentOrPathWithIdent = parse_quote!(#input_tokens);
        assert_eq!(
            input.to_token_stream().to_string(),
            input_tokens.to_string()
        );
    }
}
