use crate::__private::attribute_args::GenericsArgs;
use crate::__private::type_list::TypeList;
use darling::FromMeta;
use proc_macro2::Ident;
use syn::Path;

#[derive(FromMeta, Debug, Default, Clone, PartialEq)]
#[darling(derive_syn_parse, default)]
pub struct AutoPluginStructOrEnumAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub impl_plugin_trait: bool,
    pub impl_generic_auto_plugin_trait: bool,
    pub impl_generic_plugin_trait: bool,
}

impl GenericsArgs for AutoPluginStructOrEnumAttributeArgs {
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}

#[derive(FromMeta, Debug, Default, Clone, PartialEq)]
#[darling(derive_syn_parse, default)]
pub struct AutoPluginFnAttributeArgs {
    #[darling(multiple)]
    pub generics: Vec<TypeList>,
    pub plugin: Option<Path>,
    pub app_param: Option<Ident>,
}

impl GenericsArgs for AutoPluginFnAttributeArgs {
    const TURBOFISH: bool = true;
    fn type_lists(&self) -> &[TypeList] {
        &self.generics
    }
}
