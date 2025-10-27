use darling::{
    FromMeta,
    Result,
};
use proc_macro2::Span;
use smart_default::SmartDefault;
use std::hash::{
    Hash,
    Hasher,
};
use syn::{
    Meta,
    spanned::Spanned,
};

/// Wrapper type for darling::util::Flag that implements PartialEq and Hash
///
/// Ignores span for comparison checks
#[derive(Debug, SmartDefault, Clone)]
pub struct Flag(darling::util::Flag);

impl Flag {
    pub fn present() -> Self {
        Self(darling::util::Flag::present())
    }
    pub fn is_present(&self) -> bool {
        self.0.is_present()
    }
    pub fn span(&self) -> Span {
        self.0.span()
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_present().eq(&other.0.is_present())
    }
}

impl Eq for Flag {}

impl Hash for Flag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.is_present().hash(state);
    }
}

impl FromMeta for Flag {
    fn from_meta(meta: &Meta) -> Result<Self> {
        Ok(match meta {
            Meta::NameValue(nv) => {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(b), .. }) = &nv.value {
                    Self::from_bool(b.value)?
                } else {
                    return Err(darling::Error::unknown_value("expected boolean literal")
                        .with_span(&nv.span()));
                }
            }
            _ => Self(darling::util::Flag::from_meta(meta)?),
        })
    }
    fn from_none() -> Option<Self> {
        Some(Self(darling::util::Flag::from_none()?))
    }
    fn from_bool(value: bool) -> Result<Self> {
        Ok(Self::from(value))
    }
}

impl From<Flag> for bool {
    fn from(flag: Flag) -> Self {
        flag.is_present()
    }
}

impl From<bool> for Flag {
    fn from(v: bool) -> Self {
        if v { Self::present() } else { Self(darling::util::Flag::from(false)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    use syn::parse_quote;

    #[xtest]
    fn test_from_meta_flag_present() -> syn::Result<()> {
        assert_eq!(Flag::from_meta(&parse_quote!(this_flag))?, Flag::present());
        Ok(())
    }
    #[xtest]
    #[should_panic = "Unexpected type `lit`"]
    fn test_from_meta_flag_list_single() {
        Flag::from_meta(&parse_quote!(this_flag("foo"))).map_err(|e| e.to_string()).unwrap();
    }
    #[xtest]
    #[should_panic = "Multiple errors: (Unexpected type `lit`, Unexpected type `lit`)"]
    fn test_from_meta_flag_list_multiple() {
        Flag::from_meta(&parse_quote!(this_flag("foo", "bar"))).map_err(|e| e.to_string()).unwrap();
    }
    #[xtest]
    fn test_from_meta_flag_nv() -> syn::Result<()> {
        assert_eq!(Flag::from_meta(&parse_quote!(this_flag = true))?, Flag::present());
        assert_eq!(Flag::from_meta(&parse_quote!(this_flag = false))?, Flag::from(false));
        Ok(())
    }
}
