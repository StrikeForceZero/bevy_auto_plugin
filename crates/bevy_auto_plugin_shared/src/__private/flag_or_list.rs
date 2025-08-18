use darling::{Error, FromMeta, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use smart_default::SmartDefault;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Meta, Token};

#[derive(Debug, SmartDefault, Clone, PartialEq, Hash)]
pub struct FlagOrList<T>
where
    T: Parse,
{
    /// `true` if `#[this_flag]` or `#[this_flag(...)]` is present
    pub present: bool,
    /// The types inside `#[this_flag(...)]`, empty for the bare flag form
    pub items: Vec<T>,
}

impl<T> FlagOrList<T>
where
    T: ToTokens + Parse,
{
    pub fn to_outer_tokens(&self, flag_name: &str) -> TokenStream {
        use syn::spanned::Spanned;
        let flag_ident = Ident::new(flag_name, self.present.span());
        if self.present {
            let items = &self.items;
            if !items.is_empty() {
                quote! { #flag_ident(#(#items),*) }
            } else {
                quote! { #flag_ident }
            }
        } else {
            quote! {}
        }
    }
}

impl<T> FromMeta for FlagOrList<T>
where
    T: Parse,
{
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[this_flag]`
            Meta::Path(_) => Ok(FlagOrList {
                present: true,
                items: vec![],
            }),

            // `#[this_flag(A, B)]`
            Meta::List(list) => {
                let parsed: Punctuated<T, Token![,]> = list
                    .parse_args_with(Punctuated::parse_terminated)
                    .map_err(|e| Error::custom(e).with_span(list))?;
                Ok(FlagOrList {
                    present: true,
                    items: parsed.into_iter().collect(),
                })
            }

            // Not supported: `#[this_flag = ...]`
            Meta::NameValue(nv) => Err(Error::unsupported_format("name-value").with_span(nv)),
        }
    }
}
