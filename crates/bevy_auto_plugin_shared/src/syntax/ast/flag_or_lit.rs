use darling::{Error, FromMeta, Result};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use smart_default::SmartDefault;
use syn::spanned::Spanned;
use syn::{Expr, Lit, Meta};

#[derive(Debug, SmartDefault, Clone, PartialEq, Hash)]
pub struct FlagOrLit {
    /// `true` if `#[this_flag]` or `#[this_flag = ...]` is present
    pub present: bool,
    /// The value `#[this_flag = ...]`, empty for the bare flag form
    pub lit: Option<Lit>,
}

impl FlagOrLit {
    #[cfg(test)]
    pub fn to_outer_tokens(&self, flag_name: &str) -> TokenStream {
        use syn::spanned::Spanned;
        let flag_ident = Ident::new(flag_name, self.present.span());
        if self.present {
            let items = &self.lit;
            if let Some(lit) = items {
                quote! { #flag_ident = #lit }
            } else {
                quote! { #flag_ident }
            }
        } else {
            quote! {}
        }
    }
}

impl FromMeta for FlagOrLit {
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[this_flag]`
            Meta::Path(_) => Ok(FlagOrLit {
                present: true,
                lit: None,
            }),

            // `#[this_flag(A, B)]`
            Meta::List(list) => Err(Error::unsupported_format("list").with_span(list)),

            // Not supported: `#[this_flag = ...]`
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(lit) => Ok(FlagOrLit {
                    present: true,
                    lit: Some(lit.lit.clone()),
                }),
                other => Err(Error::unexpected_expr_type(other).with_span(&other.span())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::parse_quote;

    #[xtest]
    fn test_from_meta_flag_present() -> syn::Result<()> {
        assert_eq!(
            FlagOrLit::from_meta(&parse_quote!(this_flag))?,
            FlagOrLit {
                present: true,
                lit: None,
            }
        );
        Ok(())
    }
    #[xtest]
    fn test_from_meta_flag_set() -> syn::Result<()> {
        assert_eq!(
            FlagOrLit::from_meta(&parse_quote!(this_flag = "foo"))?,
            FlagOrLit {
                present: true,
                lit: Some(parse_quote!("foo")),
            }
        );
        Ok(())
    }
    #[xtest]
    fn test_flag_or_lit_to_outer_tokens_not_present() {
        assert_eq!(
            FlagOrLit::default()
                .to_outer_tokens("this_flag")
                .to_string(),
            quote! {}.to_string()
        )
    }
    #[xtest]
    fn test_flag_or_lit_to_outer_tokens_present() {
        assert_eq!(
            FlagOrLit {
                present: true,
                lit: None,
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag }.to_string()
        )
    }
    #[xtest]
    fn test_flag_or_lit_to_outer_tokens_set() {
        assert_eq!(
            FlagOrLit {
                present: true,
                lit: Some(parse_quote!("foo"))
            }
            .to_outer_tokens("this_flag")
            .to_string(),
            quote! { this_flag = "foo" }.to_string()
        )
    }
}
