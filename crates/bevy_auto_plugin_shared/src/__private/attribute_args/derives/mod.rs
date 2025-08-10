use darling::{FromField, FromVariant};
use proc_macro2::Ident;
use syn::Type;

pub mod auto_plugin;

#[allow(dead_code)]
#[derive(Debug, FromField)]
pub struct FieldData {
    ident: Option<Ident>,
    ty: Type,
}

#[allow(dead_code)]
#[derive(Debug, FromVariant)]
pub struct VariantData {
    ident: Ident,
    fields: darling::ast::Fields<FieldData>,
}
