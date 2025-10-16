use darling::{Error, FromMeta, Result};
use smart_default::SmartDefault;
use syn::{Expr, Meta};

#[derive(Debug, SmartDefault, Clone, PartialEq, Hash)]
pub struct FlagOrExpr {
    /// `true` if `#[this_flag]` or `#[this_flag = ...]` is present
    pub present: bool,
    /// The value `#[this_flag = ...]`, empty for the bare flag form
    pub expr: Option<Expr>,
}

impl FromMeta for FlagOrExpr {
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[this_flag]`
            Meta::Path(_) => Ok(FlagOrExpr {
                present: true,
                expr: None,
            }),

            // `#[this_flag(...)]`
            Meta::List(list) => {
                let parsed: Expr = list.parse_args().map_err(|_| {
                    Error::unsupported_format("list with multiple parameters").with_span(list)
                })?;
                Ok(FlagOrExpr {
                    present: true,
                    expr: Some(parsed),
                })
            }

            // `#[this_flag = ...]`
            Meta::NameValue(nv) => Ok(FlagOrExpr {
                present: true,
                expr: Some(nv.value.clone()),
            }),
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
            FlagOrExpr::from_meta(&parse_quote!(this_flag))?,
            FlagOrExpr {
                present: true,
                expr: None,
            }
        );
        Ok(())
    }
    #[xtest]
    fn test_from_meta_flag_list_single() -> syn::Result<()> {
        assert_eq!(
            FlagOrExpr::from_meta(&parse_quote!(this_flag("foo")))?,
            FlagOrExpr {
                present: true,
                expr: Some(parse_quote!("foo")),
            }
        );
        Ok(())
    }
    #[xtest]
    #[should_panic = "list with multiple parameters"]
    fn test_from_meta_flag_list_multiple() {
        FlagOrExpr::from_meta(&parse_quote!(this_flag("foo", "bar"))).unwrap();
    }
    #[xtest]
    fn test_from_meta_flag_nv() -> syn::Result<()> {
        assert_eq!(
            FlagOrExpr::from_meta(&parse_quote!(this_flag = "foo"))?,
            FlagOrExpr {
                present: true,
                expr: Some(parse_quote!("foo")),
            }
        );
        Ok(())
    }
}
