use darling::{
    Error,
    FromMeta,
};
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    quote,
};
use syn::{
    Ident,
    Meta,
    Token,
    Type,
    parse::Parser,
    punctuated::Punctuated,
    spanned::Spanned,
};

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TypeListEntry {
    Positional(Type),
    Named { ident: Ident, ty: Type },
}

impl TypeListEntry {
    pub fn ty(&self) -> &Type {
        match self {
            TypeListEntry::Positional(ty) => ty,
            TypeListEntry::Named { ty, .. } => ty,
        }
    }
}

impl From<Type> for TypeListEntry {
    fn from(ty: Type) -> Self {
        TypeListEntry::Positional(ty)
    }
}

impl ToTokens for TypeListEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeListEntry::Positional(ty) => ty.to_tokens(tokens),
            TypeListEntry::Named { ident, ty } => tokens.extend(quote! { #ident = #ty }),
        }
    }
}

impl syn::parse::Parse for TypeListEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let ty: Type = input.parse()?;
            Ok(TypeListEntry::Named { ident, ty })
        } else {
            let ty: Type = input.parse()?;
            Ok(TypeListEntry::Positional(ty))
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct TypeList(pub Vec<TypeListEntry>);

impl TypeList {
    pub const fn empty() -> Self {
        Self(vec![])
    }
    pub fn from_types(types: Vec<Type>) -> Self {
        Self(types.into_iter().map(TypeListEntry::from).collect())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn types_in_declared_order(&self) -> Vec<Type> {
        self.0.iter().map(|entry| entry.ty().clone()).collect()
    }
    pub fn resolve_types(&self, target_params: &[Ident]) -> syn::Result<Vec<Type>> {
        use std::collections::{
            HashMap,
            HashSet,
        };

        let target_set: HashSet<String> =
            target_params.iter().map(|ident| ident.to_string()).collect();
        let mut named_types: HashMap<String, Type> = HashMap::new();
        let mut positional_types = Vec::new();
        let mut has_named = false;

        for entry in &self.0 {
            match entry {
                TypeListEntry::Positional(ty) => positional_types.push(ty.clone()),
                TypeListEntry::Named { ident, ty } => {
                    has_named = true;
                    let key = ident.to_string();
                    if !target_set.contains(&key) {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unknown generic parameter `{}`", ident),
                        ));
                    }
                    if named_types.contains_key(&key) {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("duplicate generic parameter `{}`", ident),
                        ));
                    }
                    named_types.insert(key, ty.clone());
                }
            }
        }

        if !has_named {
            return Ok(positional_types);
        }

        let mut resolved_types = Vec::new();
        let mut positional_iter = positional_types.into_iter();
        for ident in target_params {
            let key = ident.to_string();
            if let Some(ty) = named_types.remove(&key) {
                resolved_types.push(ty);
            } else if let Some(ty) = positional_iter.next() {
                resolved_types.push(ty);
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("missing generic argument for `{}`", ident),
                ));
            }
        }

        resolved_types.extend(positional_iter);
        Ok(resolved_types)
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

fn failed_err_literal(e: syn::Error, span: &proc_macro2::Span) -> Error {
    Error::multiple(vec![
        Error::custom(
            "Failed to parse TypeList: expected a list of *types*, but found literal value(s). \
                 Use types like `Foo`, `Vec<T>`, `Option<Bar>`; not `1`, `true`, or string literals."
        ).with_span(span),
        Error::from(e),
    ])
}

fn failed_err(e: syn::Error, span: &proc_macro2::Span) -> Error {
    Error::multiple(vec![
        Error::custom("Failed to parse TypeList").with_span(span),
        Error::from(e),
    ])
}

/// Inspect `TokenStream` for `Literal`
fn contains_literal(ts: &TokenStream) -> bool {
    ts.clone().into_iter().any(|tt| matches!(tt, TokenTree::Literal(_)))
}

/// Inspect `ParseStream` for `Literal`
fn fork_has_literal(input: syn::parse::ParseStream) -> bool {
    let f = input.fork();
    let ts: TokenStream = match f.parse() {
        Ok(ts) => ts,
        Err(_) => return false,
    };
    ts.into_iter().any(|tt| matches!(tt, TokenTree::Literal(_)))
}

impl FromMeta for TypeList {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        let list = meta.require_list()?;
        // Parse its tokens as `T, T, ...` where each `T` is a syn::Type
        let parser = Punctuated::<TypeListEntry, Token![,]>::parse_terminated;
        let elems = parser.parse2(list.tokens.clone()).map_err(|e| {
            if contains_literal(&list.tokens) {
                failed_err_literal(e, &list.tokens.span())
            } else {
                failed_err(e, &list.tokens.span())
            }
        })?;
        Ok(TypeList(elems.into_iter().collect()))
    }
}

impl syn::parse::Parse for TypeList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{
            Token,
            punctuated::Punctuated,
        };
        let elems = Punctuated::<TypeListEntry, Token![,]>::parse_terminated(input)
            .map_err(|e| {
                if fork_has_literal(input) {
                    failed_err_literal(e, &input.span())
                } else {
                    failed_err(e, &input.span())
                }
            })?
            .into_iter()
            .collect();
        Ok(TypeList(elems))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::{
        Ident,
        Meta,
        Type,
        parse_quote,
        parse2,
    };

    #[derive(Debug, FromMeta)]
    #[darling(derive_syn_parse)]
    pub struct FooAttr {
        pub types: TypeList,
    }

    #[xtest]
    fn parse_types() {
        let types = quote! { u32, i32, FooBar<u32>, [u8; 4] };
        let meta: Meta = parse_quote!(types(#types));
        let attr: FooAttr = parse2(meta.to_token_stream()).unwrap();

        assert_eq!(attr.types.0.len(), 4);

        // The third element should be `Foo<u32>` with generics preserved.
        match &attr.types.0[2] {
            TypeListEntry::Positional(Type::Path(tp)) => {
                let seg = tp.path.segments.last().unwrap();
                assert_eq!(seg.ident, "FooBar");
                assert!(matches!(seg.arguments, syn::PathArguments::AngleBracketed(_)));
            }
            _ => panic!("expected Type::Path for element 2"),
        }

        let type_list = &attr.types;
        let tokens = quote! { #type_list };
        assert_eq!(tokens.to_string(), types.to_string());
    }

    #[xtest]
    fn from_meta_types() {
        let types = quote! { u32, i32, FooBar<u32>, [u8; 4] };
        let meta: Meta = parse_quote!(foo(types(#types)));
        let attr: FooAttr = FooAttr::from_meta(&meta).unwrap();

        assert_eq!(attr.types.0.len(), 4);

        // The third element should be `Foo<u32>` with generics preserved.
        match &attr.types.0[2] {
            TypeListEntry::Positional(Type::Path(tp)) => {
                let seg = tp.path.segments.last().unwrap();
                assert_eq!(seg.ident, "FooBar");
                assert!(matches!(seg.arguments, syn::PathArguments::AngleBracketed(_)));
            }
            _ => panic!("expected Type::Path for element 2"),
        }

        let type_list = &attr.types;
        let tokens = quote! { #type_list };
        assert_eq!(tokens.to_string(), types.to_string());
    }

    #[xtest]
    fn parse_named_types() {
        let types = quote! { T2 = i32, T1 = u32 };
        let meta: Meta = parse_quote!(types(#types));
        let attr: FooAttr = parse2(meta.to_token_stream()).unwrap();

        assert_eq!(attr.types.0.len(), 2);

        match &attr.types.0[0] {
            TypeListEntry::Named { ident, ty } => {
                assert_eq!(ident, "T2");
                assert_eq!(quote!(#ty).to_string(), "i32");
            }
            _ => panic!("expected TypeListEntry::Named for element 0"),
        }

        let target_params: Vec<Ident> = vec![parse_quote!(T1), parse_quote!(T2)];
        let resolved = attr.types.resolve_types(&target_params).unwrap();
        assert_eq!(quote!(#(#resolved),*).to_string(), "u32 , i32");
    }
}
