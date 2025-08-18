use darling::{Error, FromMeta, Result};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{Meta, Token};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ReflectAttr {
    /// `true` if `#[reflect]` or `#[reflect(...)]` is present
    pub present: bool,
    /// The types inside `#[reflect(...)]`, empty for the bare flag form
    pub items: Vec<Ident>,
}

impl FromMeta for ReflectAttr {
    fn from_meta(meta: &Meta) -> Result<Self> {
        match meta {
            // `#[reflect]`
            Meta::Path(_) => Ok(ReflectAttr {
                present: true,
                items: vec![],
            }),

            // `#[reflect(Debug, Foo::Bar)]`
            Meta::List(list) => {
                let parsed: Punctuated<Ident, Token![,]> = list
                    .parse_args_with(Punctuated::parse_terminated)
                    .map_err(|e| Error::custom(e).with_span(list))?;
                Ok(ReflectAttr {
                    present: true,
                    items: parsed.into_iter().collect(),
                })
            }

            // Not supported: `#[reflect = ...]`
            Meta::NameValue(nv) => Err(Error::unsupported_format("name-value").with_span(nv)),
        }
    }
}
