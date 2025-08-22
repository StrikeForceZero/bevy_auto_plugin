use darling::{Error, FromMeta};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::{Meta, Token, Type, punctuated::Punctuated};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TriggerTypeArg {
    pub event: Type,
    pub bundle: Option<Type>,
}

impl ToTokens for TriggerTypeArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut types = vec![&self.event];
        if let Some(bundle) = &self.bundle {
            types.push(bundle);
        }
        tokens.extend(quote! { Trigger < #(#types),* > });
    }
}

impl From<&TriggerTypeArg> for TokenStream {
    fn from(value: &TriggerTypeArg) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl From<TriggerTypeArg> for TokenStream {
    fn from(value: TriggerTypeArg) -> Self {
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        tokens
    }
}

impl FromMeta for TriggerTypeArg {
    fn from_meta(meta: &Meta) -> Result<Self, Error> {
        let list = meta.require_list()?;
        // Parse its tokens as `T, T, ...` where each `T` is a syn::Type
        let parser = Punctuated::<Type, Token![,]>::parse_terminated;
        let elems = parser.parse2(list.tokens.clone()).map_err(Error::custom)?;
        let mut elems = elems.into_iter();
        let Some(elem) = elems.next() else {
            return Err(Error::custom(
                "expected 1-2 arg - trigger(event) or trigger(event, bundle)",
            ));
        };
        let trigger = TriggerTypeArg {
            event: elem,
            bundle: elems.next(),
        };
        if elems.next().is_some() {
            return Err(Error::custom(
                "unexpected 3rd arg - trigger(event, bundle, _)",
            ));
        };
        Ok(trigger)
    }
}

impl syn::parse::Parse for TriggerTypeArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elems = Punctuated::<Type, Token![,]>::parse_terminated(input)?;
        let mut elems = elems.into_iter();
        let Some(elem) = elems.next() else {
            return Err(syn::Error::new(
                input.span(),
                "expected 1-2 arg - trigger(event) or trigger(event, bundle)",
            ));
        };
        let trigger = TriggerTypeArg {
            event: elem,
            bundle: elems.next(),
        };
        if let Some(third) = elems.next() {
            return Err(syn::Error::new(
                third.span(),
                "unexpected 3rd arg - trigger(event, bundle, _)",
            ));
        };
        Ok(trigger)
    }
}
