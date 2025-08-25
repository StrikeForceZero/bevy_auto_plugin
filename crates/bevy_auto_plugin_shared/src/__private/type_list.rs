use crate::__private::util::generics_traits::{CountGenerics, HasGenericsCollection};
use darling::{Error, FromMeta};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::Parser;
use syn::{Meta, Token, Type, punctuated::Punctuated};

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct TypeList(pub Vec<Type>);

impl TypeList {
    pub const fn empty() -> Self {
        Self(vec![])
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl ToTokens for TypeList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let types = &self.0;
        let new_tokens = quote! { #(#types),* };
        tokens.extend(new_tokens);
    }
}

impl From<&TypeList> for TokenStream {
    fn from(list: &TypeList) -> Self {
        let mut tokens = TokenStream::new();
        list.to_tokens(&mut tokens);
        tokens
    }
}

impl From<TypeList> for TokenStream {
    fn from(list: TypeList) -> Self {
        let mut tokens = TokenStream::new();
        list.to_tokens(&mut tokens);
        tokens
    }
}

impl FromMeta for TypeList {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        let list = meta.require_list()?;
        // Parse its tokens as `T, T, ...` where each `T` is a syn::Type
        let parser = Punctuated::<Type, Token![,]>::parse_terminated;
        let elems = parser.parse2(list.tokens.clone()).map_err(Error::custom)?;
        Ok(TypeList(elems.into_iter().collect()))
    }
}

impl syn::parse::Parse for TypeList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{Token, Type, punctuated::Punctuated};
        let elems = Punctuated::<Type, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(TypeList(elems))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{Meta, Type, parse_quote};

    #[derive(Debug, FromMeta)]
    pub struct FooAttr {
        pub types: TypeList,
    }

    #[internal_test_proc_macro::xtest]
    fn parse_types() {
        let types = quote! { u32, i32, FooBar<u32>, [u8; 4] };
        let meta: Meta = parse_quote!(foo(types(#types)));
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();

        assert_eq!(attr.types.0.len(), 4);

        // The third element should be `Foo<u32>` with generics preserved.
        match &attr.types.0[2] {
            Type::Path(tp) => {
                let seg = tp.path.segments.last().unwrap();
                assert_eq!(seg.ident, "FooBar");
                assert!(matches!(
                    seg.arguments,
                    syn::PathArguments::AngleBracketed(_)
                ));
            }
            _ => panic!("expected Type::Path for element 2"),
        }

        let type_list = &attr.types;
        let tokens = quote! { #type_list };
        assert_eq!(tokens.to_string(), types.to_string());
    }
}

impl CountGenerics for TypeList {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.span()
    }

    fn count_generics(&self) -> syn::Result<usize> {
        Ok(self.len())
    }
}

impl HasGenericsCollection for TypeList {
    type CollectionItem = Self;
    type Collection = Vec<Self>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![self.clone()])
    }
}
