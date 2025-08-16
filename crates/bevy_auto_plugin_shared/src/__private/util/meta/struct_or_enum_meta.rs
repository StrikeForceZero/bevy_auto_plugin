use crate::__private::util::generics_traits::{CountGenerics, HasGenericsCollection};
use crate::__private::util::meta::IdentGenericsAttrsMeta;
use proc_macro2::{Ident, Span};
use syn::{Attribute, Error, Generics, Item};

pub struct StructOrEnumMeta<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub attributes: &'a [Attribute],
}

impl<'a> StructOrEnumMeta<'a> {
    fn new(ident: &'a Ident, generics: &'a Generics, attributes: &'a [Attribute]) -> Self {
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
    fn ident(&self) -> &'a Ident {
        self.ident
    }
    fn generics(&self) -> &'a Generics {
        self.generics
    }
    fn attributes(&self) -> &'a [Attribute] {
        self.attributes
    }
}

impl<'a> HasGenericsCollection for StructOrEnumMeta<'a> {
    type CollectionItem = &'a syn::Generics;
    type Collection = Vec<&'a syn::Generics>;
    fn generics(&self) -> syn::Result<Self::Collection> {
        Ok(vec![self.generics])
    }
}

impl CountGenerics for StructOrEnumMeta<'_> {
    fn get_span(&self) -> Span {
        use syn::spanned::Spanned;
        self.generics.span()
    }

    fn count_generics(&self) -> syn::Result<usize> {
        Ok(self.generics.params.len())
    }
}
