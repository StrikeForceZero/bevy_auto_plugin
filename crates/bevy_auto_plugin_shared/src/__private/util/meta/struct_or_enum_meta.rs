use crate::__private::util::meta::IdentGenericsAttrsMeta;
use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

pub struct StructOrEnumMeta<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a Vec<Attribute>,
}

impl<'a> StructOrEnumMeta<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a Vec<Attribute>) -> Self {
        Self {
            ident,
            generics,
            attributes,
        }
    }
}

impl<'a> TryFrom<&'a Item> for StructOrEnumMeta<'a> {
    type Error = Error;

    fn try_from(item: &'a Item) -> std::result::Result<Self, Self::Error> {
        use syn::spanned::Spanned;
        Ok(match item {
            Item::Struct(struct_item) => StructOrEnumMeta::new(
                &struct_item.ident,
                &struct_item.generics,
                &struct_item.attrs,
            ),
            Item::Enum(enum_item) => {
                StructOrEnumMeta::new(&enum_item.ident, &enum_item.generics, &enum_item.attrs)
            }
            _ => return Err(Error::new(item.span(), "expected struct or enum")),
        })
    }
}

impl<'a> IdentGenericsAttrsMeta<'a> for StructOrEnumMeta<'a> {
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
