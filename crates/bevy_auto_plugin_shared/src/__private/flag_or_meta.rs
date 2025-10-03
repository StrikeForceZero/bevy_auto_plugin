use darling::{Error, FromMeta, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use smart_default::SmartDefault;
use syn::Meta;
use syn::parse::Parse;

#[derive(Debug, SmartDefault, Clone, PartialEq, Hash)]
pub struct FlagOrMeta<T>
where
    T: FromMeta + Parse,
{
    /// `true` if `#[this_flag]` or `#[this_flag(...)]` is present
    pub present: bool,
    /// The inner meta inside `#[this_flag(...)]`, None for the bare flag form
    pub inner_meta: Option<T>,
}

impl<T> FlagOrMeta<T>
where
    T: ToTokens + FromMeta + Parse,
{
    pub fn to_outer_tokens(&self, flag_name: &str) -> TokenStream {
        use syn::spanned::Spanned;
        let flag_ident = Ident::new(flag_name, self.present.span());
        if self.present {
            let items = &self.inner_meta;
            if let Some(items) = items {
                quote! { #flag_ident(#items) }
            } else {
                quote! { #flag_ident }
            }
        } else {
            quote! {}
        }
    }
}

impl<T> FromMeta for FlagOrMeta<T>
where
    T: FromMeta + Parse,
{
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[this_flag]`
            Meta::Path(_) => Ok(FlagOrMeta {
                present: true,
                inner_meta: None,
            }),

            // `#[this_flag(A, B)]`
            Meta::List(_) => {
                // (T::from_meta sees Meta::List and can do `list.tokens` parsing inside)
                let t = T::from_meta(meta)?;
                Ok(Self {
                    present: true,
                    inner_meta: Some(t),
                })
            }

            // Not supported: `#[this_flag = ...]`
            Meta::NameValue(nv) => Err(Error::unsupported_format("name-value").with_span(nv)),
        }
    }
}

impl<T> Parse for FlagOrMeta<T>
where
    T: FromMeta + Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // #[flag] -> no tokens after the key
        if input.is_empty() {
            return Ok(Self {
                present: true,
                inner_meta: None,
            });
        }

        // #[flag(...)] -> parenthesized payload parsed as T
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let inner: T = content.parse()?;
            // ensure no extra tokens after the group
            if !input.is_empty() {
                return Err(input.error("unexpected tokens after parenthesized payload"));
            }
            return Ok(Self {
                present: true,
                inner_meta: Some(inner),
            });
        }

        Err(input.error("expected nothing or a parenthesized payload"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(FromMeta, Default, Debug, Copy, Clone, PartialEq)]
    #[darling(derive_syn_parse, default)]
    struct Test {
        pub a: Option<u32>,
        pub b: Option<u32>,
    }

    impl ToTokens for Test {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let mut items = vec![];
            if let Some(a) = self.a {
                items.push(quote! { a = #a });
            }
            if let Some(b) = self.b {
                items.push(quote! { b = #b });
            }
            tokens.extend(quote! { #(#items),* });
        }
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_not_present() {
        assert_eq!(
            FlagOrMeta::<Ident>::default()
                .to_outer_tokens("this_flag")
                .to_string(),
            quote! {}.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_empty() {
        assert_eq!(
            FlagOrMeta::<Ident> {
                present: true,
                inner_meta: None,
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag }.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_single_item() {
        assert_eq!(
            FlagOrMeta::<Test> {
                present: true,
                inner_meta: Some(Test {
                    a: Some(1),
                    b: None,
                }),
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag(a = 1u32) }.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_to_outer_tokens_multiple_item() {
        assert_eq!(
            FlagOrMeta::<Test> {
                present: true,
                inner_meta: Some(Test {
                    a: Some(1),
                    b: Some(2),
                }),
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag(a = 1u32, b = 2u32) }.to_string()
        )
    }

    #[internal_test_proc_macro::xtest]
    fn test_flag_or_list_parsing() {
        let input = quote! { (a = 1u32, b = 2u32) };
        let item = syn::parse2::<FlagOrMeta<Test>>(input).map_err(|e| e.to_string());
        let expected = Ok(FlagOrMeta::<Test> {
            present: true,
            inner_meta: Some(Test {
                a: Some(1),
                b: Some(2),
            }),
        });
        assert_eq!(item, expected);
    }
}
