use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::{Expr, Token, punctuated::Punctuated};

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ExprPathOrCall {
    Path(syn::ExprPath),
    Call(syn::ExprCall),
}

impl ToTokens for ExprPathOrCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprPathOrCall::Path(path) => path.to_tokens(tokens),
            ExprPathOrCall::Call(call) => call.to_tokens(tokens),
        }
    }
}

impl From<&ExprPathOrCall> for TokenStream {
    fn from(value: &ExprPathOrCall) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl From<ExprPathOrCall> for TokenStream {
    fn from(value: ExprPathOrCall) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl FromMeta for ExprPathOrCall {
    fn from_meta(meta: &syn::Meta) -> Result<Self, darling::Error> {
        fn one_expr_to_variant(expr: Expr) -> Result<ExprPathOrCall, darling::Error> {
            // allow parens like `item = (Foo(bar))`
            let expr = match expr {
                Expr::Paren(p) => *p.expr,
                other => other,
            };
            match expr {
                Expr::Path(p) => Ok(ExprPathOrCall::Path(p)),
                Expr::Call(c) => Ok(ExprPathOrCall::Call(c)),
                other => Err(darling::Error::custom(format!(
                    "unsupported expression: expected path or call, got `{}`",
                    other.to_token_stream()
                ))),
            }
        }

        match meta {
            // Support: `item(...)`
            syn::Meta::List(list) => {
                // Parse exactly one Expr from the list's tokens.
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                let elems = parser
                    .parse2(list.tokens.clone())
                    .map_err(darling::Error::custom)?;
                let mut it = elems.into_iter();
                let expr = it
                    .next()
                    .ok_or_else(|| darling::Error::custom("expected one expression"))?;
                if it.next().is_some() {
                    return Err(darling::Error::custom("expected exactly one expression"));
                }
                one_expr_to_variant(expr)
            }

            // Support: `item = <expr>`
            syn::Meta::NameValue(nv) => {
                // syn v2 uses Expr for the RHS of name-value meta.
                one_expr_to_variant(nv.value.clone())
            }

            // Bare flag like `item` is not accepted.
            syn::Meta::Path(_) => Err(darling::Error::custom(
                "expected `item = <expr>` or `item(<expr>)`",
            )),
        }
    }
}

impl syn::parse::Parse for ExprPathOrCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elems = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        let mut elems = elems.into_iter();
        let Some(elem) = elems.next() else {
            return Err(syn::Error::new(
                input.span(),
                "expected exactly one path or call",
            ));
        };
        if elems.next().is_some() {
            return Err(syn::Error::new(
                input.span(),
                "expected exactly one path or call",
            ));
        };
        Ok(match elem {
            Expr::Call(call) => ExprPathOrCall::Call(call),
            Expr::Path(path) => ExprPathOrCall::Path(path),
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    "unsupported expression - expected path or call",
                ));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{Meta, parse_quote};

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Foo(usize);

    #[derive(Debug, FromMeta)]
    pub struct FooAttr {
        pub item: ExprPathOrCall,
    }

    #[test]
    fn from_call() {
        let expr = parse_quote! { Foo(1) };
        let meta: Meta = parse_quote!(foo(item(#expr)));
        let expr_path_or_call = ExprPathOrCall::Call(expr);
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();

        let item = attr.item;
        assert_eq!(item, expr_path_or_call);

        assert_eq!(
            item.to_token_stream().to_string(),
            expr_path_or_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn from_path() {
        let expr = parse_quote! { ::bar::Foo };
        let meta: Meta = parse_quote!(foo(item(#expr)));
        let expr_path_or_call = ExprPathOrCall::Path(expr);
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();

        let item = attr.item;
        assert_eq!(item, expr_path_or_call);

        assert_eq!(
            item.to_token_stream().to_string(),
            expr_path_or_call.to_token_stream().to_string()
        );
    }

    #[test]
    fn from_call_eq() {
        let expr: Expr = syn::parse_quote! { OnEnter(SomeState::Foo) };
        let meta: Meta = syn::parse_quote!(attr(item = #expr));
        #[derive(FromMeta)]
        struct FooAttr {
            item: ExprPathOrCall,
        }
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();
        assert!(matches!(attr.item, ExprPathOrCall::Call(_)));
    }

    #[test]
    fn from_path_eq() {
        let expr: Expr = syn::parse_quote! { ::some::path::Update };
        let meta: Meta = syn::parse_quote!(attr(item = #expr));
        #[derive(FromMeta)]
        struct FooAttr {
            item: ExprPathOrCall,
        }
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();
        assert!(matches!(attr.item, ExprPathOrCall::Path(_)));
    }
}
