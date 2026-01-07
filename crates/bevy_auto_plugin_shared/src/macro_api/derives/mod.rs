use darling::{
    FromField,
    FromVariant,
};
use proc_macro2::Ident;
use syn::Type;

mod auto_plugin;

#[allow(dead_code)]
#[derive(Debug, FromField)]
pub struct FieldData {
    pub ident: Option<Ident>,
    pub ty: Type,
}

#[allow(dead_code)]
#[derive(Debug, FromVariant)]
pub struct VariantData {
    pub ident: Ident,
    pub fields: darling::ast::Fields<FieldData>,
}

pub mod prelude {

    pub use super::auto_plugin::*;
}
