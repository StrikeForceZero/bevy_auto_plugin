use crate::__private::util::meta::IdentGenericsAttrsMeta;
use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

pub struct FnMeta<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a Vec<Attribute>,
}

impl<'a> FnMeta<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a Vec<Attribute>) -> Self {
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

impl<'a> IdentGenericsAttrsMeta<'a> for FnMeta<'a> {
    fn ident(&self) -> &Ident {
        self.ident
    }
    fn generics(&self) -> &Generics {
        self.generics
    }
    fn attributes(&self) -> &Vec<Attribute> {
        self.attributes
    }
}
