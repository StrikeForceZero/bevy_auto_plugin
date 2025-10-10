use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

pub struct FnMeta<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a [Attribute],
}

impl<'a> FnMeta<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a [Attribute]) -> Self {
        Self {
            ident,
            generics,
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Item> for FnMeta<'a> {
    type Error = Error;

    fn try_from(item: &'a Item) -> std::result::Result<Self, Self::Error> {
        use syn::spanned::Spanned;
        Ok(match item {
            Item::Fn(fn_item) => {
                Self::new(&fn_item.sig.ident, &fn_item.sig.generics, &fn_item.attrs)
            }
            _ => return Err(Error::new(item.span(), "expected fn")),
        })
    }
}
