use proc_macro2::Ident;
use syn::{Attribute, Error, Generics, Item};

pub mod fn_meta;
pub mod struct_or_enum_meta;

pub trait IdentGenericsAttrsMeta<'a>: TryFrom<&'a Item, Error = Error> {
    fn ident(&self) -> &'a Ident;
    fn generics(&self) -> &'a Generics;
    fn attributes(&self) -> &'a [Attribute];
}
