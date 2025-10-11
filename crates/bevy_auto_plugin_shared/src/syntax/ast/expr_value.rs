use darling::{Error, FromMeta};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{Expr, Meta, Token, punctuated::Punctuated};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ExprValue(pub Expr);

impl ToTokens for ExprValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let closure = &self.0;
        closure.to_tokens(tokens)
    }
}

impl From<&ExprValue> for TokenStream {
    fn from(value: &ExprValue) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl From<ExprValue> for TokenStream {
    fn from(value: ExprValue) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

fn failed_err(e: syn::Error, span: &proc_macro2::Span) -> Error {
    Error::multiple(vec![
        Error::custom("failed to parse ExprValue").with_span(span),
        Error::from(e),
    ])
}

impl FromMeta for ExprValue {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        let list = meta.require_list()?;
        // Parse its tokens as `T, T, ...` where each `T` is a syn::Type
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        let elems = parser
            .parse2(list.tokens.clone())
            .map_err(|e| failed_err(e, &list.tokens.span()))?;
        let mut elems = elems.into_iter();
        let Some(elem) = elems.next() else {
            return Err(Error::too_few_items(1).with_span(&meta.span()));
        };
        if let Some(elem) = elems.next() {
            return Err(Error::too_many_items(1).with_span(&elem.span()));
        };
        Ok(ExprValue(elem))
    }
}

impl syn::parse::Parse for ExprValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elems = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        let mut elems = elems.into_iter();
        let Some(elem) = elems.next() else {
            return Err(Error::too_few_items(1).with_span(&input.span()).into());
        };
        if elems.next().is_some() {
            return Err(Error::too_many_items(1).with_span(&input.span()).into());
        };
        Ok(ExprValue(elem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use quote::quote;
    use syn::{Meta, parse_quote};

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Foo(usize);

    #[derive(Debug, FromMeta)]
    pub struct FooAttr {
        pub item: ExprValue,
    }

    #[xtest]
    fn parse_types() {
        let expr = parse_quote! { Foo(1) };
        let meta: Meta = parse_quote!(foo(item(#expr)));
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();

        let item = attr.item;
        assert_eq!(item.0, expr);

        let tokens = quote! { #item };
        assert_eq!(tokens.to_string(), expr.to_token_stream().to_string());
    }
}
