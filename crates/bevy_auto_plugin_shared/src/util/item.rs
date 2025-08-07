use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

pub fn require_fn(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only functions use this attribute macro",
        )),
    }
}

pub fn require_struct_or_enum(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only struct and enum can use this attribute macro",
        )),
    }
}

pub trait IdentGenericsAttrs<'a>: TryFrom<&'a Item, Error = Error> {
    fn ident(&self) -> &Ident;
    fn generics(&self) -> &Generics;
    fn attributes(&self) -> &Vec<Attribute>;
}
