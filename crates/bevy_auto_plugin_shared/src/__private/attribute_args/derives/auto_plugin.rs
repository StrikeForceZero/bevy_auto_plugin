use crate::__private::attribute_args::attributes::modes::global::auto_plugin::AutoPluginStructOrEnumAttributeArgs;
use crate::__private::attribute_args::derives::{FieldData, VariantData};
use darling::FromDeriveInput;
use proc_macro2::Ident;
use syn::{Attribute, Generics, Visibility};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(auto_plugin), forward_attrs, supports(struct_any, enum_any))]
pub struct GlobalAutoPluginDeriveArgs {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: darling::ast::Data<VariantData, FieldData>,
    pub attrs: Vec<Attribute>,
    #[darling(flatten)]
    pub auto_plugin: AutoPluginStructOrEnumAttributeArgs,
}
