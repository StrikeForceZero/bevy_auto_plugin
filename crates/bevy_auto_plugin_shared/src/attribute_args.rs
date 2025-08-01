use crate::type_list::TypeList;
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::Ident;
use syn::{Attribute, Generics, Path, Type, Visibility};

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

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(auto_plugin), forward_attrs, supports(struct_any, enum_any))]
pub struct GlobalAutoPluginDeriveParams {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub data: darling::ast::Data<VariantData, FieldData>,
    pub attrs: Vec<Attribute>,
    #[darling(flatten)]
    pub auto_plugin: GlobalAutoPluginStructOrEnumAttributeParams,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginStructOrEnumAttributeParams {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub impl_plugin_trait: bool,
    pub impl_generic_auto_plugin_trait: bool,
    pub impl_generic_plugin_trait: bool,
}

#[derive(FromMeta, Debug, Default)]
#[darling(derive_syn_parse, default)]
pub struct GlobalAutoPluginFnAttributeParams {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
}

#[derive(FromMeta, Debug, PartialEq, Hash)]
#[darling(derive_syn_parse)]
pub struct GlobalStructOrEnumAttributeParams {
    pub plugin: Path,
    #[darling(flatten, default)]
    pub inner: StructOrEnumAttributeParams,
}

impl GlobalStructOrEnumAttributeParams {
    pub fn has_generics(&self) -> bool {
        self.inner.has_generics()
    }
    fn concat_ident_hash(&self, ident: &Ident) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ident.hash(&mut hasher);
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn get_unique_ident_string(&self, prefix: &'static str, ident: &Ident) -> String {
        let hash = self.concat_ident_hash(ident);
        format!("{prefix}_{hash}")
    }

    pub fn get_unique_ident(&self, prefix: &'static str, ident: &Ident) -> Ident {
        let ident_string = self.get_unique_ident_string(prefix, ident);
        Ident::new(&ident_string, ident.span())
    }
}

#[derive(FromMeta, Debug, Default, PartialEq, Hash)]
#[darling(derive_syn_parse, default)]
pub struct StructOrEnumAttributeParams {
    // TODO: #[darling(multiple)]
    //     pub generics: Vec<TypeList>,
    pub generics: Option<TypeList>,
}

impl StructOrEnumAttributeParams {
    pub fn has_generics(&self) -> bool {
        self.generics
            .as_ref()
            .map(|types| !types.0.is_empty())
            .unwrap_or(false)
    }
}
