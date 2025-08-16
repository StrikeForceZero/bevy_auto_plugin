use proc_macro2::Ident;
use syn::{Error, Item};

pub fn resolve_ident_from_fn(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Fn(f) => Ok(&f.sig.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only functions use this attribute macro",
        )),
    }
}

pub fn resolve_ident_from_struct_or_enum(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Struct(s) => Ok(&s.ident),
        Item::Enum(e) => Ok(&e.ident),
        _ => Err(Error::new_spanned(
            item,
            "Only struct and enum can use this attribute macro",
        )),
    }
}
