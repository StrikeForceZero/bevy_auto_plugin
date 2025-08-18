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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_not_present() {
        assert_eq!(
            FlagOrList::<Ident>::default()
                .to_outer_tokens("this_flag")
                .to_string(),
            quote! {}.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_empty() {
        assert_eq!(
            FlagOrList::<Ident> {
                present: true,
                items: vec![]
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag }.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_single_item() {
        assert_eq!(
            FlagOrList::<Ident> {
                present: true,
                items: vec![parse_quote!(A)]
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag(A) }.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_multiple_item() {
        assert_eq!(
            FlagOrList::<Ident> {
                present: true,
                items: vec![parse_quote!(A), parse_quote!(B)]
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag(A, B) }.to_string()
        )
    }
}
